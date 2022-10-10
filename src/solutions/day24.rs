use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let components: Vec<(u16, u16)> = input
        .trim()
        .lines()
        .map(|l| {
            l.split("/")
                .map(|x| x.parse::<u16>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let mut picked: Vec<usize> = Vec::new();
    let mut sockets: Vec<u16> = vec![0];
    let max_weight = dfs(&components, &mut picked, &mut sockets);

    (
        max_weight.to_string(),
        String::from("<not yet implemented>"),
    )
}

fn dfs(components: &Vec<(u16, u16)>, picked: &mut Vec<usize>, sockets: &mut Vec<u16>) -> u32 {
    let socket = *sockets.last().unwrap();
    let mut weight = 0;
    for (i, component) in components.iter().enumerate() {
        if picked.contains(&i) {
            continue;
        }
        if component.0 == socket {
            picked.push(i);
            sockets.push(component.1);
            weight =
                weight.max((component.0 + component.1) as u32 + dfs(components, picked, sockets));
            picked.pop();
            sockets.pop();
        } else if component.1 == socket {
            picked.push(i);
            sockets.push(component.0);
            weight =
                weight.max((component.0 + component.1) as u32 + dfs(components, picked, sockets));
            picked.pop();
            sockets.pop();
        }
    }
    weight
}
