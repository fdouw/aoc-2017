use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let components: Vec<(usize, (u16, u16, u32))> = input
        .trim()
        .lines()
        .map(|l| {
            l.split("/")
                .map(|x| x.parse::<u16>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .map(|(a, b)| (a, b, (a + b) as u32))
        .enumerate()
        .collect();

    let mut picked: Vec<bool> = Vec::with_capacity(components.len());
    (0..components.len()).for_each(|_| picked.push(false));
    let max_weight = dfs(&components, &mut picked, 0);

    (
        max_weight.to_string(),
        String::from("<not yet implemented>"),
    )
}

fn dfs(components: &Vec<(usize, (u16, u16, u32))>, picked: &mut Vec<bool>, port: u16) -> u32 {
    let mut weight = 0;
    for (i, component) in components {
        if !picked[*i] {
            if component.0 == port {
                picked[*i] = true;
                weight = weight.max(component.2 + dfs(components, picked, component.1));
                picked[*i] = false;
            } else if component.1 == port {
                picked[*i] = true;
                weight = weight.max(component.2 + dfs(components, picked, component.0));
                picked[*i] = false;
            }
        }
    }
    weight
}
