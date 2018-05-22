pub fn email_body_activation(activation_token: &str) -> String {
    format!("
LYnLab 계정 등록을 위한 인증 이메일입니다.

아래의 링크를 눌러 인증 절차를 진행해주세요.
만약 링크가 눌러지지 않는다면, 브라우저의 주소창에 복사하여 진행해주세요.

https://accounts.lynlab.co.kr/activate/{activation_token}

만약 본인이 가입한 것이 아니라면 이 메일을 무시하셔도 됩니다.
    ", activation_token=activation_token)
}
