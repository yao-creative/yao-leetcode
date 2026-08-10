struct Flatten<I>
where
    I: Iterator,
    I::Item: Iterator,
{
    outer: I,
    // your state goes here
    

}

impl<I> Flatten<I>
where
    I: Iterator,
    I::Item: Iterator,
{
    fn new(iter: I) -> Self {
        Self {
            outer: iter,
            // ...
        }
    }
}

impl<I> Iterator for Flatten<I>
where
    I: Iterator,
    I::Item: Iterator,
{
    type Item = I::Item::Item;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatten_finite() {
        let nested = vec![
            vec![1, 2, 3],
            vec![4, 5],
            vec![6],
        ];

        let result: Vec<_> = Flatten::new(
            nested.into_iter().map(|x| x.into_iter())
        )
        .collect();

        assert_eq!(
            result,
            vec![1, 2, 3, 4, 5, 6]
        );
    }


    #[test]
    fn flatten_infinite_outer() {
        let nested = std::iter::repeat_with(|| {
            vec![1, 2, 3].into_iter()
        });

        let result: Vec<_> = Flatten::new(nested)
            .take(10)
            .collect();

        assert_eq!(
            result,
            vec![1,2,3,1,2,3,1,2,3,1]
        );
    }


    #[test]
    fn flatten_empty_inner() {
        let nested = vec![
            vec![],
            vec![1],
            vec![],
            vec![2,3],
        ];

        let result: Vec<_> = Flatten::new(
            nested.into_iter().map(|x| x.into_iter())
        )
        .collect();

        assert_eq!(
            result,
            vec![1,2,3]
        );
    }
}