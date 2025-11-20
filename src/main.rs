
// write down...
// run consume
// async로 계속 돌 수 있나?
use serde::Deserialize;

struct Test {
    id: String,
    val: u32,
}

impl Test {
    fn set_robot(r_id:String, r_val:u32) -> Self {
        Self{
            id: r_id,
            val: r_val
        }
    }

    fn println(self){
        println!("id: {}, val: {}", self.id, self.val);
    }
}

//요구사항 정리
// async로 동작필요. 따라서 main은 async로 동작할 수 있는 eventloop 같은 걸 계속 돌려야 함
// async loop 에서는 Kafka에서 데이터를 가져오고, handler에 각 data type 별로 처리 방식을 나누어야 함
// Kafka에서 들어오는 데이터는 JSON이므로 이를 파싱하고 처리할 방법 필요

#[tokio::main]
async fn main() {
    let r = Test::set_robot("robot".to_string(), 10);
    r.println();

    //serde_json::Deserializer
}
