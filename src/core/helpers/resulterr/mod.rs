use framework_cqrs_lib::cqrs::models::errors::ResultErr;

pub trait SanitizeVecResultErr<T: Clone> {
    fn sanitize_vec_result_err(&self) -> ResultErr<Vec<T>>;
}

impl<T: Clone> SanitizeVecResultErr<T> for Vec<ResultErr<T>> {
    fn sanitize_vec_result_err(&self) -> ResultErr<Vec<T>> {
        self
            .into_iter()
            .fold(Ok(vec![]), |accr, itemr| {
                match (accr, itemr) {
                    (Ok(acc), Ok(item)) => {
                        let wrapper = vec![item.clone()];
                        Ok([&acc[..], &wrapper[..]].concat())
                    }
                    (Ok(_), Err(item)) => {
                        Err(item.clone())
                    }
                    (Err(acc), _) => {
                        Err(acc)
                    }
                }
            })
    }
}