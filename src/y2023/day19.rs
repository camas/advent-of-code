use std::{collections::HashMap, ops::Range};

use winnow::{
    ascii::{alpha1, digit1, multispace0},
    combinator::{alt, separated},
    Parser,
};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let (workflows, ratings) = parse_input(input);

    let workflow_map = workflows
        .iter()
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect::<HashMap<_, _>>();

    let mut part1 = 0;
    for rating in ratings {
        let mut workflow_name = "in";
        loop {
            let workflow = workflow_map.get(workflow_name).unwrap();
            match workflow.evaluate(&rating) {
                Destination::Accept => {
                    part1 +=
                        rating.extremely_cool + rating.musical + rating.aerodynamic + rating.shiny;
                    break;
                }
                Destination::Reject => break,
                Destination::Workflow(name) => {
                    workflow_name = name;
                }
            }
        }
    }

    let mut inputs_per_workflow: HashMap<String, Vec<String>> = HashMap::new();
    for workflow in workflows.iter() {
        for destination in workflow
            .rules
            .iter()
            .map(|rule| &rule.destination)
            .chain(std::iter::once(&workflow.default))
        {
            if let Destination::Workflow(name) = destination {
                inputs_per_workflow
                    .entry(name.clone())
                    .or_default()
                    .push(workflow.name.clone());
            }
        }
    }

    let mut ratings_per_workflow: HashMap<String, Vec<RangePartRating>> = HashMap::new();
    ratings_per_workflow.insert(
        "in".to_string(),
        vec![RangePartRating {
            extremely_cool: 1..4001,
            musical: 1..4001,
            aerodynamic: 1..4001,
            shiny: 1..4001,
        }],
    );

    let mut part2 = 0;
    while !ratings_per_workflow.is_empty() {
        let workflow_name = ratings_per_workflow
            .keys()
            .find(|workflow_name| {
                inputs_per_workflow
                    .get(*workflow_name)
                    .filter(|inputs| !inputs.is_empty())
                    .is_none()
            })
            .unwrap();
        inputs_per_workflow
            .values_mut()
            .for_each(|inputs| inputs.retain(|input| input != workflow_name));

        let workflow = workflow_map.get(workflow_name).unwrap();
        let ratings = ratings_per_workflow.remove(&workflow_name.clone()).unwrap();
        for rating in ratings {
            for (new_rating, destination) in workflow.evaluate_range(rating) {
                match destination {
                    Destination::Accept => {
                        part2 += (new_rating.extremely_cool.end - new_rating.extremely_cool.start)
                            * (new_rating.musical.end - new_rating.musical.start)
                            * (new_rating.aerodynamic.end - new_rating.aerodynamic.start)
                            * (new_rating.shiny.end - new_rating.shiny.start);
                    }
                    Destination::Reject => (),
                    Destination::Workflow(new_workflow_name) => {
                        ratings_per_workflow
                            .entry(new_workflow_name)
                            .or_default()
                            .push(new_rating);
                    }
                }
            }
        }
    }

    (part1, part2)
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default: Destination,
}

#[derive(Debug)]
struct Rule {
    category: Category,
    operation: Operation,
    operation_value: i64,
    destination: Destination,
}

#[derive(Debug, Clone)]
enum Destination {
    Workflow(String),
    Accept,
    Reject,
}

#[derive(Debug, Clone, Copy)]
enum Category {
    ExtremelyCool,
    Aerodynamic,
    Musical,
    Shiny,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    GreaterThan,
    LessThan,
}

#[derive(Debug)]
struct PartRating {
    extremely_cool: i64,
    musical: i64,
    aerodynamic: i64,
    shiny: i64,
}

#[derive(Debug, Clone)]
struct RangePartRating {
    extremely_cool: Range<i64>,
    musical: Range<i64>,
    aerodynamic: Range<i64>,
    shiny: Range<i64>,
}

impl Workflow {
    fn evaluate(&self, rating: &PartRating) -> &Destination {
        for rule in self.rules.iter() {
            let passes_rule = match rule.operation {
                Operation::GreaterThan => rating.category(rule.category) > rule.operation_value,
                Operation::LessThan => rating.category(rule.category) < rule.operation_value,
            };
            if passes_rule {
                return &rule.destination;
            }
        }
        &self.default
    }

    fn evaluate_range(&self, mut rating: RangePartRating) -> Vec<(RangePartRating, Destination)> {
        let mut results = Vec::new();

        for rule in self.rules.iter() {
            match rule.operation {
                Operation::GreaterThan => {
                    if rating.category(rule.category).end > rule.operation_value + 1 {
                        if rating.category(rule.category).start > rule.operation_value {
                            results.push((rating, rule.destination.clone()));
                            return results;
                        } else {
                            let mut new_rating = rating.clone();
                            new_rating.category_mut(rule.category).start = rule.operation_value + 1;
                            rating.category_mut(rule.category).end = rule.operation_value + 1;
                            results.push((new_rating, rule.destination.clone()));
                        }
                    }
                }
                Operation::LessThan => {
                    if rating.category(rule.category).start < rule.operation_value {
                        if rating.category(rule.category).end < rule.operation_value {
                            results.push((rating, rule.destination.clone()));
                            return results;
                        } else {
                            let mut new_rating = rating.clone();
                            new_rating.category_mut(rule.category).end = rule.operation_value;
                            rating.category_mut(rule.category).start = rule.operation_value;
                            results.push((new_rating, rule.destination.clone()));
                        }
                    }
                }
            }
        }

        results.push((rating, self.default.clone()));

        results
    }
}

impl PartRating {
    fn category(&self, category: Category) -> i64 {
        match category {
            Category::ExtremelyCool => self.extremely_cool,
            Category::Musical => self.musical,
            Category::Aerodynamic => self.aerodynamic,
            Category::Shiny => self.shiny,
        }
    }
}

impl RangePartRating {
    fn category(&self, category: Category) -> &Range<i64> {
        match category {
            Category::ExtremelyCool => &self.extremely_cool,
            Category::Musical => &self.musical,
            Category::Aerodynamic => &self.aerodynamic,
            Category::Shiny => &self.shiny,
        }
    }

    fn category_mut(&mut self, category: Category) -> &mut Range<i64> {
        match category {
            Category::ExtremelyCool => &mut self.extremely_cool,
            Category::Musical => &mut self.musical,
            Category::Aerodynamic => &mut self.aerodynamic,
            Category::Shiny => &mut self.shiny,
        }
    }
}

fn parse_input(input: &str) -> (Vec<Workflow>, Vec<PartRating>) {
    (
        separated(1.., parse_workflow, "\n"),
        "\n\n",
        separated(1.., parse_rating, "\n"),
        multispace0,
    )
        .map(|(workflows, _, ratings, _)| (workflows, ratings))
        .parse(input)
        .unwrap()
}

fn parse_workflow(input: &mut &str) -> winnow::Result<Workflow> {
    (
        alpha1,
        "{",
        separated(1.., parse_rule, ","),
        ",",
        parse_destination,
        "}",
    )
        .map(|(name, _, rules, _, default, _)| Workflow {
            name: name.to_string(),
            rules,
            default,
        })
        .parse_next(input)
}

fn parse_rule(input: &mut &str) -> winnow::Result<Rule> {
    (
        parse_category,
        parse_operation,
        parse_number,
        ":",
        parse_destination,
    )
        .map(
            |(category, operation, operation_value, _, destination)| Rule {
                category,
                operation,
                operation_value,
                destination,
            },
        )
        .parse_next(input)
}

fn parse_destination(input: &mut &str) -> winnow::Result<Destination> {
    alt((
        "R".map(|_| Destination::Reject),
        "A".map(|_| Destination::Accept),
        alpha1.map(|name: &str| Destination::Workflow(name.to_string())),
    ))
    .parse_next(input)
}

fn parse_operation(input: &mut &str) -> winnow::Result<Operation> {
    alt((
        ">".map(|_| Operation::GreaterThan),
        "<".map(|_| Operation::LessThan),
    ))
    .parse_next(input)
}

fn parse_rating(input: &mut &str) -> winnow::Result<PartRating> {
    (
        "{x=",
        parse_number,
        ",m=",
        parse_number,
        ",a=",
        parse_number,
        ",s=",
        parse_number,
        "}",
    )
        .map(
            |(_, extremely_cool, _, musical, _, aerodynamic, _, shiny, _)| PartRating {
                extremely_cool,
                aerodynamic,
                musical,
                shiny,
            },
        )
        .parse_next(input)
}

fn parse_category(input: &mut &str) -> winnow::Result<Category> {
    alt((
        "x".map(|_| Category::ExtremelyCool),
        "m".map(|_| Category::Musical),
        "a".map(|_| Category::Aerodynamic),
        "s".map(|_| Category::Shiny),
    ))
    .parse_next(input)
}

fn parse_number(input: &mut &str) -> winnow::Result<i64> {
    digit1
        .map(|digits: &str| digits.parse::<i64>().unwrap())
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

        let (part1, part2) = solve(input);

        assert_eq!(part1.to_string(), "19114");
        assert_eq!(part2.to_string(), "167409079868000");
    }
}
