use std::path::Path;

struct Window<const N: usize> {
    chars: [char; N],
}

impl<const N: usize> Window<N> {
    fn new(initial: [char; N]) -> Self {
        Self { chars: initial }
    }

    fn drop_first_and_add(&mut self, c: char) {
        self.chars.copy_within(1.., 0);
        self.chars[N - 1] = c;
    }

    fn unique(&self) -> bool {
        let mut answer = true;
        for i in 0..N {
            for j in (i + 1)..N {
                answer &= self.chars[i] != self.chars[j];
            }
        }
        answer
    }
}

pub async fn part<const LEN: usize>(path: impl AsRef<Path>) -> anyhow::Result<usize> {
    let contents = tokio::fs::read_to_string(path).await?;

    let initial: [char; LEN] = contents
        .chars()
        .take(LEN)
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| anyhow::anyhow!("File did not have at least {} characters!", LEN))?;
    let mut window = Window::<LEN>::new(initial);
    if window.unique() {
        return Ok(4);
    }
    for (i, c) in contents
        .chars()
        .enumerate()
        .map(|(i, c)| (i + 1, c))
        .skip(LEN)
    {
        window.drop_first_and_add(c);
        if window.unique() {
            return Ok(i);
        }
    }
    Err(anyhow::anyhow!(
        "There were not four consecutive different characters in the input!"
    ))
}
