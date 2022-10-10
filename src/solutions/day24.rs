use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let components: Vec<(usize, (u16, u16, u16))> = input
        .trim()
        .lines()
        .map(|l| {
            l.split("/")
                .map(|x| x.parse::<u16>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .map(|(a, b)| (a, b, a + b))
        .enumerate()
        .collect();

    let mut picked: Vec<bool> = Vec::with_capacity(components.len());
    (0..components.len()).for_each(|_| picked.push(false));
    let (weight_part1, weight_part2, _) = dfs(&components, &mut picked, 0);

    (weight_part1.to_string(), weight_part2.to_string())
}

fn dfs(
    components: &Vec<(usize, (u16, u16, u16))>,
    picked: &mut Vec<bool>,
    port: u16,
) -> (u16, u16, usize) {
    let mut weight1 = 0;
    let mut weight2 = 0;
    let mut len = 0;
    for (i, component) in components {
        if !picked[*i] {
            if component.0 == port {
                picked[*i] = true;

                let (w1, w2, l) = dfs(components, picked, component.1);
                weight1 = weight1.max(component.2 + w1);
                if l > len || (l == len && component.2 + w2 > weight2) {
                    weight2 = component.2 + w2;
                    len = l;
                }

                picked[*i] = false;
            } else if component.1 == port {
                picked[*i] = true;

                let (w1, w2, l) = dfs(components, picked, component.0);
                weight1 = weight1.max(component.2 + w1);
                if l > len || (l == len && component.2 + w2 > weight2) {
                    weight2 = component.2 + w2;
                    len = l;
                }

                picked[*i] = false;
            }
        }
    }
    // len + 1, to account for current component
    (weight1, weight2, len + 1)
}
