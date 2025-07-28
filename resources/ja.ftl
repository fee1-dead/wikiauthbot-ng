-privacy_info = WikiAuthBot2の[プライバシーポリシー](<https://wikiauthbot-ng.toolforge.org/ps>)をご覧いただき、情報の利用方法をご確認ください。

auth = 以下のリンクからWikimediaアカウントで認証してください: [認証する]({$url})

    {-privacy_info}

auth_exists_in_server = このサーバーではすでに認証済みです。再度認証する必要はありません。

auth_to_server = あなたはすでに [{$name}](<{$url}>) として認証されています。このサーバーでも認証しますか？

    {-privacy_info}

yes = はい
no = いいえ

auth_footer = このリンクの有効期限は5分です。

authreq_canceled = 認証がキャンセルされました。
authreq_expired = 認証の有効期限が切れました。

authlog = {$mention} さんが [利用者:{$username}](<{$user_link}>)（ID {$wmf_id}）として認証されました。

# 認証された理由の監査ログ
auditlog_successful_auth = Wikimediaユーザー {$wmf_id} として認証されました。

authreq_successful = 認証に成功しました。

bot = WikiAuthBot

whois_no_user_found = ユーザーが見つかりませんでした。このサーバーにいないか、認証されていません。

revwhois_fail = 指定されたユーザー情報を取得できませんでした。ユーザー名が正しいかご確認ください。

revwhois_no_auth = [{$name}](<{$user_link}>) はこのサーバーで認証されていません。

revwhois_one = [{$name}](<{$user_link}>) は {$mention} に認証されています。

# 注意:コロンと変数の間にスペースなし
revwhois_multiple = [{$name}](<{$user_link}>) は以下のアカウントに認証されています:{$mentions}

user_link = https://ja.wikipedia.org/w/index.php?title=Special%3ACentralAuth/{$normalized_name}

welcome_has_auth = ようこそ {$mention} さん！すでに [{$name}](<{$user_link}>) として認証されていますので、再認証は不要です。

welcome_has_auth_failed = ようこそ {$mention} さん！すでに認証されています（情報取得エラー）、再認証は不要です。

welcome = ようこそ {$mention} さん！Wikimediaアカウントの認証したい場合は、</auth:1241068923730919464> を入力またはクリックしてください。

whois_global_groups = グローバルグループ: {$groupslist}

whois_blocked = **ブロックされています**
whois_locked = **ロックされています**
whois_pblocked = 部分ブロック
whois_edits = 編集回数: {$edits}
whois_groups = グループ: {$groupslist}
whois_overflow = 最大10件まで表示。全情報を見るには上部の名前をクリックしてください。
whois_no_block_reason = <!-- 理由はありません -->

# 日付形式をYYYY-MM-DD以外にしたい場合はご相談ください。
whois = Discord: {$mention}
    登録日: {$registration}
    ホーム: {$home}
    {$global_groups}総編集回数: {$edits}

cancel = キャンセル

deauth = このサーバーで認証を解除してもよろしいですか？
deauth_canceled = 認証解除がキャンセルされました。
deauth_expired = 認証解除の有効期限が切れました。
deauth_not_found = 現在このサーバーで認証されていません。認証済みのサーバーでコマンドを実行してください。
deauth_done = 認証データの削除が完了しました。
deauth_more = 現在 {$num_servers_authed} サーバーで認証されています。データをこのサーバーのみ、または全てのサーバーから削除しますか？
deauth_more_single = このサーバーのみから削除
deauth_more_single_done = このサーバーから認証データの削除が完了しました。
deauth_more_multi = 参加している全てのサーバーから削除
deauth_more_multi_done = {$num_servers_authed} サーバーから認証データの削除が完了しました。
deauth_log = {$mention} さんがこのサーバーで認証解除しました。
deauth_audit_log = 認証解除済み

auth_failed_blocked = 認証失敗: Wikimediaプロジェクトのいずれかでブロックされているため、このサーバーで認証できません。サーバー管理者にご連絡ください。
auth_failed_error = 認証失敗: 内部エラーが発生しました。バグ報告はDiscordの beef.w さんまでお願いします。
removed_blocked_user_roles = ブロックされたユーザーからロールを削除しました。
adding_managed_role = 条件を満たしたため、Bot管理ロールを追加します。
removing_managed_role = 条件を満たさなくなったため、Bot管理ロールを削除します。

server_auth_success = 成功！認証情報がBotに送信されました :)

cmd_whois = whois
cmd_whois_desc = 認証済みメンバーのアカウント詳細を確認
cmd_whois_user = ユーザー
cmd_whois_user_desc = 確認したい利用者。空欄なら自分のアカウント
cmd_whois_menu = whoisを取得
cmd_auth = 認証
cmd_auth_desc = Wikimediaアカウントで認証
cmd_revwhois = revwhois
cmd_revwhois_desc = Wikimediaアカウントに紐づくDiscordアカウント一覧
cmd_revwhois_user = ユーザー
cmd_revwhois_user_desc = Wikimediaユーザー名
cmd_deauth = 認証解除
cmd_deauth_desc = Botから認証解除またはデータ削除