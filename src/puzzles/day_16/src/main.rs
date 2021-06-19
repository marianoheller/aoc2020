use regex::Regex;
use utils::read_lines;

type Ticket = Vec<usize>;
type ParsedTicket = Vec<(String, usize)>;

type Contraints = Vec<(String, Vec<(usize, usize)>)>;

fn get_invalids(ticket: &Ticket, constraints: &Contraints) -> Vec<usize> {
    ticket
        .clone()
        .into_iter()
        .filter(|v| {
            !constraints.iter().any(|(_, field_constraint)| {
                field_constraint
                    .iter()
                    .any(|(low, high)| (v) >= (low) && (v) <= (high))
            })
        })
        .collect::<Vec<usize>>()
}

fn parse_ticket(
    ticket: &Ticket,
    other_tickets: &Vec<Ticket>,
    constraints: &Contraints,
) -> ParsedTicket {
    let mut all_tickets = other_tickets.clone();
    all_tickets.push(ticket.clone());

    let mut left_constraints = constraints.clone();
    let mut parsed_ticket: ParsedTicket = vec![];

    while parsed_ticket.len() < ticket.len() {
        for (col_num, og_ticket_col_value) in ticket.into_iter().enumerate() {
            let col_values = all_tickets
                .iter()
                .map(|ticket| ticket[col_num])
                .collect::<Vec<_>>();

            let found_fields: Vec<(String, usize)> = left_constraints
                .iter()
                .filter(|(_name, cons)| {
                    col_values.iter().all(|col_value| {
                        cons.iter()
                            .any(|(low, high)| col_value >= low && col_value <= high)
                    })
                })
                .map(|(name, _)| (name.to_owned(), og_ticket_col_value.clone()))
                .collect();

            if found_fields.len() == 1 {
                let to_be_saved = found_fields[0].clone();
                left_constraints = left_constraints
                    .into_iter()
                    .filter(|(name, _)| *name != to_be_saved.0)
                    .collect::<Vec<_>>();
                parsed_ticket.push(to_be_saved);
            }
        }
    }

    parsed_ticket
}

fn parse_input(lines: Vec<String>) -> (Contraints, Ticket, Vec<Ticket>) {
    let mut iter = lines.split(|line| line == "");
    let lines_constraints = iter.next().unwrap();
    let lines_tickets = iter.next().unwrap();
    let lines_other_tickets = iter.next().unwrap();

    // contraints
    let re = Regex::new(r#"^([a-zA-Z ]+): (\d+)-(\d+) or (\d+)-(\d+)$"#).unwrap();
    let contraints = lines_constraints
        .into_iter()
        .cloned()
        .map(|line| {
            let cap = re.captures_iter(&line[..]).next().unwrap();
            let c1 = (
                cap[2].parse::<usize>().unwrap(),
                cap[3].parse::<usize>().unwrap(),
            );
            let c2 = (
                cap[4].parse::<usize>().unwrap(),
                cap[5].parse::<usize>().unwrap(),
            );

            (cap[1].to_owned(), vec![c1, c2])
        })
        .collect::<Contraints>();

    // ticket
    let ticket = lines_tickets[1]
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    // other tickets
    let other_tickets = lines_other_tickets
        .iter()
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|v| (v).parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Ticket>>();

    (contraints, ticket, other_tickets)
}

fn main() {
    let lines = read_lines("src/inputs/day_16.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();

    let (constraints, ticket, other_tickets) = parse_input(lines);

    let invalids = other_tickets
        .iter()
        .flat_map(|ticket| get_invalids(ticket, &constraints))
        .collect::<Vec<_>>();
    println!("Part 1: {:?}", invalids.into_iter().sum::<usize>());

    let other_valid_tickets = other_tickets
        .into_iter()
        .filter(|ticket| get_invalids(ticket, &constraints).len() == 0)
        .collect::<Vec<_>>();

    let result = parse_ticket(&ticket, &other_valid_tickets, &constraints);
    println!("Part 2: {:?}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example_vals_p1() -> (Contraints, Ticket, Vec<Ticket>) {
        let constraints: Contraints = vec![
            ("class".to_owned(), vec![(1, 3), (5, 7)]),
            ("row".to_owned(), vec![(6, 11), (33, 44)]),
            ("seat".to_owned(), vec![(13, 40), (45, 50)]),
        ]
        .into_iter()
        .collect();

        let ticket = vec![11, 12, 13];

        let tickets = vec![
            vec![7, 3, 47],
            vec![40, 4, 50],
            vec![55, 2, 20],
            vec![38, 6, 12],
        ];

        (constraints, ticket, tickets)
    }

    fn get_example_vals_p2() -> (Contraints, Ticket, Vec<Ticket>) {
        let constraints: Contraints = vec![
            ("class".to_owned(), vec![(0, 1), (4, 19)]),
            ("row".to_owned(), vec![(0, 5), (8, 19)]),
            ("seat".to_owned(), vec![(0, 13), (16, 19)]),
        ]
        .into_iter()
        .collect();

        let ticket = vec![11, 12, 13];

        let tickets = vec![vec![3, 9, 18], vec![15, 1, 5], vec![5, 14, 9]];

        (constraints, ticket, tickets)
    }

    #[test]
    fn example_p1() {
        let (constraints, _, other_tickets) = get_example_vals_p1();

        let invalids: Vec<usize> = other_tickets
            .iter()
            .flat_map(|ticket| get_invalids(ticket, &constraints))
            .collect();

        assert_eq!(invalids, vec![4, 55, 12]);
    }

    #[test]
    fn example_p2() {
        let (constraints, ticket, other_tickets) = get_example_vals_p2();

        let other_valid_tickets = other_tickets
            .into_iter()
            .filter(|ticket| get_invalids(ticket, &constraints).len() == 0)
            .collect::<Vec<_>>();

        let result = parse_ticket(&&ticket, &other_valid_tickets, &constraints);

        assert_eq!(
            result,
            vec![
                ("row".to_owned(), 11),
                ("class".to_owned(), 12),
                ("seat".to_owned(), 13)
            ]
        );
    }
}
