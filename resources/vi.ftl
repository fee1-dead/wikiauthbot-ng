-privacy_info = See WikiAuthBot2's [privacy statement](<https://wikiauthbot-ng.toolforge.org/ps>) to learn more about how we use your information.

auth = Vui lòng sử dụng liên kết này để xác thực tài khoản Wikimedia của bạn: [Đăng nhập]({$url})

    {-privacy_info}

auth_exists_in_server = Bạn đã xác thực tài khoản của bạn trong máy chủ này, vì vậy bạn không cần phải liên kết lại nữa.

auth_to_server = Bạn đã đăng nhập trước đó bằng tên [{$name}](<{$url}>). Bạn có muốn liên kết nó vào máy chủ này không?

    {-privacy_info}

yes = Có
no = Không

auth_footer = Liên kết này sẽ hết hạn sau 5 phút.

authreq_canceled = Xác thực bị hủy.
authreq_expired = Xác thực đã hết hạn.

authlog = {$mention} đã đăng nhập bằng tên người dùng [Thành viên:{$username}](<{$user_link}>) (id {$wmf_id})

# The entry in the audit log recording why the authenticated role was granted.
auditlog_successful_auth = đã đăng nhập bằng tên người dùng {$wmf_id} trên Wikimedia

authreq_successful = Xác thực thành công.

bot = WikiAuthBot

whois_no_user_found = Không tìm thấy thành viên này. Có thể thành viên này không có mặt trong máy chủ hoặc chưa xác thực.

revwhois_fail = Không thể truy xuất thông tin. Xin hãy đảm bảo rằng bạn đã điền đúng tên người dùng.

revwhois_no_auth = [{$name}](<{$user_link}>) chưa liên kết trong máy chủ này.

revwhois_one = [{$name}](<{$user_link}>) được liên kết với {$mention}

# note: no space between colon and variable.
revwhois_multiple = [{$name}](<{$user_link}>) được xác thực cho tài khoản sau:{$mentions}

user_link = https://vi.wikipedia.org/w/index.php?title=Special%3ACentralAuth/{$normalized_name}

welcome_has_auth = Hoan nghênh {$mention}! Bạn trước đó đã liên kết với tên [{$name}](<{$user_link}>), vì vậy bạn không cần phải liên kết lại.

welcome_has_auth_failed = Hoan nghênh {$mention}! Bạn đã liên kết tài khoản trước đó (có lỗi khi truy xuất thông tin), vì vậy bạn không cần phải liên kết lại.

welcome = Hoan nghênh {$mention}! Nếu bạn muốn xác thực (liên kết tài khoản wiki), vui lòng nhập </auth:1241068923730919464>

whois_global_groups = Nhóm toàn cục: {$groupslist}

whois_blocked = **CẤM**
whois_locked = **KHÓA TOÀN CỤC**
whois_pblocked = bị cấm bán phần
whois_edits = Số sửa đổi: {$edits}
whois_groups = Nhóm quyền: {$groupslist}
whois_overflow = Chỉ hiển thị tối đa 10 wiki có số sửa đổi nhiều nhất. Nhấn vào tên người dùng ở trên để xem thêm.
whois_no_block_reason = <!-- Chưa cung cấp lý do -->

# If you need a different date format other than YYYY-MM-DD, let me know.
whois = Discord: {$mention}
    Mở tài khoản: {$registration}
    Wiki nhà: {$home}
    Nhóm toàn cục: {$global_groups}
    Tổng số sửa đổi toàn cục: {$edits}

cancel = hủy

deauth = Bạn có muốn hủy liên kết tài khoản khỏi máy chủ này?
deauth_canceled = Thao tác này đã bị hủy.
deauth_expired = Thao tác hủy liên kết này đã hết hạn.
deauth_not_found = Bạn chưa liên kết tài khoản trong máy chủ này. Bạn chỉ có thể chạy lệnh này tại máy chủ mà bạn đã liên kết.
deauth_done = Đã xóa thông tin xác thực tài khoản.
deauth_more = Bạn đã xác thực trong {$num_servers_authed} máy chủ. Bạn có muốn chỉ xóa dữ liệu liên kết ra khỏi máy chủ này, hay xóa ra khỏi tất cả {$num_servers_authed} máy chủ?
deauth_more_single = Chỉ xóa dữ liệu ra khỏi máy chủ này
deauth_more_single_done = Đã xóa thông tin xác thực ra khỏi máy chủ này.
deauth_more_multi = Xóa ra khỏi tất cả máy chủ mà tôi đang có mặt
deauth_more_multi_done = Đã xóa thông tin xác thực ra khỏi tất cả {$num_servers_authed} máy chủ.
deauth_log = {$mention} đã hủy liên kết khỏi máy chủ này.
deauth_audit_log = Thành viên đã hủy liên kết

auth_failed_blocked = Xác thực thất bại: Bạn đang bị cấm trên một hoặc nhiều dự án Wikimedia, điều đó ngăn cản bạn không được xác thực trong máy chủ này. Liên hệ với quản trị viên máy chủ để được trợ giúp.
auth_failed_error = Xác thực thất bại: Có lỗi nội bộ đã xảy ra. Vui lòng liên hệ beef.w trên Discord để báo cáo lỗi.
removed_blocked_user_roles = Gỡ vai trò khỏi thành viên bị cấm
adding_managed_role = Thêm vai trò (tự động) cho thành viên đáp ứng (các) tiêu chí
removing_managed_role = Gỡ vai trò (tự động) từ thành viên không đáp ứng (các) tiêu chí

server_auth_success = Thành công! Thông tin xác thực đã được gửi đến bot :)

cmd_whois = whois
cmd_whois_desc = Kiểm tra thông tin tài khoản của thành viên đã liên kết
cmd_whois_user = user
cmd_whois_user_desc = Tên người dùng Discord để kiểm tra, để trống để kiểm tra chính bạn
cmd_whois_menu = Lấy thông tin
cmd_auth = auth
cmd_auth_desc = Liên kết tài khoản Discord với tài khoản Wikimedia
cmd_revwhois = revwhois
cmd_revwhois_desc = Liệt kê tài khoản Discord được liên kết với tài khoản Wikimedia
cmd_revwhois_user = user
cmd_revwhois_user_desc = Tên người dùng Wikimedia
cmd_deauth = deauth
cmd_deauth_desc = Hủy liên kết hoặc xóa bỏ dữ liệu khỏi bot.
