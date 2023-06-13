pub fn job_route(job_id: &str, token: &str) -> String {
    format!("/job/{}?token={}", &job_id, token)
}
