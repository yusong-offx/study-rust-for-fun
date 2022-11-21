# study-rust-for-fun
+ lifetime annotation

고루틴 고쓰레드가 sleep 러스트 진짜 쓰레드 sleep
+ tokio 같은 다른 비동기 runtime 실행해야함  
- 상시 tokio 위에서 구동시켜야함으로 코딩시 주의요함  
가령 std::thread::sleep 사용시 진짜 쓰레드 block, tokio::time::sleep 사용

스마트 포인터
+ 단일쓰레드 : Rc\<T> / Rc<RefcCell\<T>>
+ 멀티쓰레드 : Arc\<T> / Arc<Mutex\<T>>