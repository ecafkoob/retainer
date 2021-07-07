use retainer::Cache;
use simple_logger::SimpleLogger;

use std::time::Duration;

#[tokio::main]
async fn main() {
    // enable logs for example purposes
    // 启动一个 logger 接受 cache 的 log.
    // 问题来了, 这个 log 是以怎样的形式运行 单独起了一个线程? 还是通过什么样的形式呢? 每当产生一个log
    // 是怎么发送到 logger 的.
    // SimpleLogger 实现了 log::Log trait.
    // 实现了这个 trait 就可以通过set_logger这类相关的方法把一个全局 logger 变成一个有静态生命周期的 Log
    // trait object.
    // 然后通过 log 的各种宏就可以打日志了. 这里使用门脸模式. 门脸对外表现得功能是一样的.但是后面使用的 log
    // 实现可以是不同的.
    let x = SimpleLogger::new().init().unwrap();


    // create our new cache
    let cache = Cache::new();

    // insert 100K entries
    // 之所以叫异步缓存,异步就体现在这里了.看到没插入数据是很慢的 io 操作. 整成异步之后. 数据的同步效率就
    // 提高了.要不然这里可能还要加个 q 来处理积压的问题.妈耶. 真滴麻烦.
    for i in 0..100000 {
        cache.insert(i, i, Duration::from_millis(i)).await;
    }

    // spawn a monitor using Redis config; 20 keys every 100ms
    cache.monitor(20, 0.25, Duration::from_millis(100)).await;
}
