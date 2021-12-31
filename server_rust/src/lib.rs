// #![feature(stmt_expr_attributes)]
#![allow(non_upper_case_globals)]

pub mod data_access;
pub mod data_access1;
pub mod permission;

use permission::{ApiFlag, FnDisplay, FnFlag, Role, RoleFn, User, UserRole};

#[rustfmt::skip]
pub const fn_flags: [FnFlag; 15] = [
  FnFlag::new(1, 0, 0, 1 << 0, "user_management", "用户管理", FnDisplay::Show ),
    FnFlag::new(101, 1, 1, 1 << 1, "wx_user:disable", "微信用户", FnDisplay::Disable),
    FnFlag::new(102, 1, 1, 1 << 2, "wx_user:show", "微信用户", FnDisplay::Show),
    //   FnFlag::new(10101, 101, 2, 1 << 1, "获取列表"  , FnDisplay::Show), // 这是 Api 的事
      FnFlag::new(10101, 101, 2, 1 << 1, "set_user_tag:disable", "设置用户组", FnDisplay::Disable),
      FnFlag::new(10102, 101, 2, 1 << 2, "set_user_tag:show", "设置用户组", FnDisplay::Show),
      FnFlag::new(10103, 101, 2, 1 << 3, "", "详细资料"  , FnDisplay::Show),
      FnFlag::new(10104, 101, 2, 1 << 4, "", "设置备注"  , FnDisplay::Show),
    FnFlag::new(104, 1, 1, 1 << 4, "", "用户分组管理", FnDisplay::Show),
    //   FnFlag::new(10401, 104, 3, 1 << 1, "", "获取列表", FnDisplay::Show), // 这是 Api 的事
      FnFlag::new(10401, 104, 3, 1 << 1, "", "新增"    , FnDisplay::Disable),
      FnFlag::new(10402, 104, 3, 1 << 2, "", "新增"    , FnDisplay::Show),
      FnFlag::new(10402, 104, 3, 1 << 3, "", "导出用户", FnDisplay::Show),
      FnFlag::new(10402, 104, 3, 1 << 4, "", "编辑"    , FnDisplay::Show),
  FnFlag::new(2, 0, 0, 1 << 1, "", "素材管理", FnDisplay::Show),
  FnFlag::new(3, 0, 0, 1 << 2, "", "红包管理", FnDisplay::Show),
  FnFlag::new(4, 0, 0, 1 << 3, "", "报表", FnDisplay::Show),
];

#[rustfmt::skip]
pub const api_flags: [ApiFlag; 8] = [
    ApiFlag::new(10101, 101, 2, 1 << 1, "获取列表"  , "wx_user/get_list"),
    ApiFlag::new(10102, 101, 2, 1 << 2, "设置用户组", "wx_user/set_user_tag"),
    ApiFlag::new(10103, 101, 2, 1 << 3, "详细资料"  , "wx_user/get_detail"),
    ApiFlag::new(10104, 101, 2, 1 << 4, "设置备注"  , "wx_user/set_remark"),

    ApiFlag::new(10401, 104, 3, 1 << 1, "获取列表", "user_tag/get_detail"),
    ApiFlag::new(10402, 104, 3, 1 << 2, "新增"    , "user_tag/add"),
    ApiFlag::new(10402, 104, 3, 1 << 3, "导出用户", "user_tag/export"),
    ApiFlag::new(10402, 104, 3, 1 << 4, "编辑"    , "user_tag/update"),
];

#[rustfmt::skip]
pub const roles: [Role; 5] = [
    Role { id: 1, name: "用户管理" }, // 用户管理的所有页面都有权限
    Role { id: 2, name: "微信用户管理" },
    Role { id: 3, name: "用户分组管理" },
    Role { id: 4, name: "微信用户查看" },
    Role { id: 5, name: "用户分组查看" },
];

#[rustfmt::skip]
pub const role_fns: [RoleFn; 16] = [
    // 用户管理
    RoleFn { role_id: 1, seq: 0, value: 0b1  /* 用户管理 */},
    RoleFn { role_id: 1, seq: 1, value: 0b110  /* 微信用户 | 用户分组管理*/},
    RoleFn { role_id: 1, seq: 2, value: 0b11110 /* 微信用户/四个功能都有 */ },
    RoleFn { role_id: 1, seq: 3, value: 0b11110 /* 用户分组管理/四个功能都有 */ },
    // 微信用户管理
    RoleFn { role_id: 2, seq: 0, value: 0b1  /* 用户管理 */},
    RoleFn { role_id: 2, seq: 1, value: 0b10  /* 微信用户 */},
    RoleFn { role_id: 2, seq: 2, value: 0b11110 /* 四个功能都有 */ },
    // 用户分组管理
    RoleFn { role_id: 3, seq: 0, value: 0b1  /* 用户管理 */},
    RoleFn { role_id: 3, seq: 1, value: 0b100  /* 用户分组管理 */},
    RoleFn { role_id: 3, seq: 3, value: 0b11110 /* 四个功能都有 */ },
    // 微信用户查看
    RoleFn { role_id: 4, seq: 0, value: 0b1  /* 用户管理 */},
    RoleFn { role_id: 4, seq: 1, value: 0b10 /* 微信用户 */},
    RoleFn { role_id: 4, seq: 2, value: 0b10 /* 获取列表 */},
    // 用户分组查看
    RoleFn { role_id: 5, seq: 0, value: 0b1  /* 用户管理 */},
    RoleFn { role_id: 5, seq: 1, value: 0b100  /* 用户分组管理 */},
    RoleFn { role_id: 5, seq: 3, value: 0b10 /* 获取列表 */ },
];

pub const users: [User; 3] = [
    User { id: 1, name: "AA" },
    User { id: 2, name: "BB" },
    User { id: 3, name: "CC" },
];

#[rustfmt::skip]
pub const user_roles: [UserRole; 5] = [
    // AA: 微信用户管理，用户分组查看
    UserRole { user_id: 1, role_id: 2, },
    UserRole { user_id: 1, role_id: 5, },
    // BB: 用户管理
    UserRole { user_id: 2, role_id: 1, },
    // CC: 微信用户管理，用户分组管理
    UserRole { user_id: 3, role_id: 2, },
    UserRole { user_id: 3, role_id: 3, },
];

type HashMap = std::collections::HashMap<u32, u64>;
pub fn get_user_permission(user_id: u32) -> HashMap {
    let map = user_roles
        .iter()
        .filter(|&ur| ur.user_id == user_id)
        .flat_map(|ur| roles.iter().filter(|&r| r.id == ur.role_id))
        .flat_map(|r| role_fns.iter().filter(|&rf| rf.role_id == r.id))
        .fold(HashMap::new(), |mut hs: HashMap, rf: &RoleFn| {
            *hs.entry(rf.seq).or_default() |= rf.value;
            hs
        });
    map
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_flag_tree() {
        let map = &super::get_user_permission(2);
        println!("{:?}", map)
    }
}
