pub const SENDER_ADDRESS: &'static str = "GETTO Example <labo@message.getto.systems>";

pub mod send_reset_token {
    pub const SUBJECT: &'static str = "GETTO Example パスワードリセットの件 [URL のご案内]";
    pub const BODY: &'static str = r#####"お世話になっております
GETTO Example システムです

パスワードリセットの申請を受け付けましたので
リセットのための URL を送信いたします

下記 URL より、パスワードのリセットができます

{URL}

パスワードリセットをキャンセルする場合は
単にこのメールを無視してください

もしパスワードリセットの申請をしていない場合は
お手数ですが、管理者に連絡をお願いいたします

よろしくお願いいたします

#################################
GETTO Example
email: labo@message.getto.systems
#################################
"#####;
}

pub mod notify_password_reset {
    pub const SUBJECT: &'static str = "GETTO Example パスワードリセットの件 [リセット完了のお知らせ]";
    pub const BODY: &'static str = r#####"お世話になっております
GETTO Example システムです

パスワードリセットが完了しました
今後は新しいパスワードを使用してログインしてください

もしパスワードリセットをしていない場合は
アカウント不正使用の可能性がありますので管理者にご連絡ください

よろしくお願いいたします

#################################
GETTO Example
email: labo@message.getto.systems
#################################
"#####;
}
