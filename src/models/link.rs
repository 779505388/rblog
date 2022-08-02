use crate::RB;
use rbatis::{crud::CRUD, crud_table, DateTimeNative};

use serde::{Deserialize, Serialize};

#[crud_table(table_name:link)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Link {
    pub id: Option<usize>,
    pub name: Option<String>,
    pub link: Option<String>,
    pub avatar: Option<String>,
    pub brief: Option<String>,
    pub created: Option<DateTimeNative>,
}

impl Link {
    //获取所有友链
    pub async fn get_all() -> Result<Vec<Link>, rbatis::Error> {
        RB.fetch_list::<Link>().await
    }

    //提交新友链
    pub async fn commit_link(data: Link) -> bool {
        let r = RB.save(&data, &[]).await;
        match r {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    //更新友链
    pub async fn update_link(data: Link) -> bool {
        let r = RB.update_by_column::<Link>("id",&data).await;
        match r {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    //删除友链
    pub async fn delete_link(data: Link) -> bool {
        let r = RB.remove_by_column::<Link,_>("id",&data.id).await;
        match r {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
