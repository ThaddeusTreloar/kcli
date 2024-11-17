struct ErrBuf {
    buf: [u8; 512],
}

struct NativeEvent {
    inner: NonNull<rd_kafka_op_s>,
}

impl NativeEvent {
    fn ptr(&self) -> *mut rdkafka_sys::rd_kafka_op_s {
        self.inner.as_ptr()
    }
}

struct ListConsumerGroupsFuture {
    rx: oneshot::Receiver<NativeEvent>,
}

impl Future for ListConsumerGroupsFuture {
    type Output = KafkaResult<()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let event = ready!(self.rx.poll_unpin(cx)).map_err(|_| KafkaError::Canceled)?;

        let res = unsafe { rdkafka_sys::rd_kafka_event_ListConsumerGroups_result(event.ptr()) };
        if res.is_null() {
            let typ = unsafe { rdkafka_sys::rd_kafka_event_type(event.ptr()) };
            return Poll::Ready(Err(KafkaError::AdminOpCreation(format!(
                "delete topics request received response of incorrect type ({})",
                typ
            ))));
        }
        let mut n = 0;
        let groups = unsafe { rdkafka_sys::rd_kafka_ListConsumerGroups_result_valid(res, &mut n) };

        println!("We got it");

        Poll::Ready(Ok(()))
    }
}
/*
let admin_client = ClientConfig::new()
    .set("bootstrap.servers", "localhost:9092")
    .set_log_level(RDKafkaLogLevel::Emerg)
    .create::<AdminClient<DefaultClientContext>>()
    .expect("Failed to create client.");

let client_ptr = admin_client.inner().native_ptr();

let native_opts = unsafe {
    rdkafka_sys::rd_kafka_AdminOptions_new(
        client_ptr,
        rdkafka_sys::RDKafkaAdminOp::RD_KAFKA_ADMIN_OP_ANY,
    )
};

let mut err_buf = ErrBuf { buf: [0; 512] };

let res = unsafe {
    rdkafka_sys::rd_kafka_AdminOptions_set_request_timeout(
        native_opts,
        Duration::from_millis(1000).as_millis() as i32,
        err_buf.buf.as_mut_ptr() as *mut c_char,
        512,
    )
};

println!("{:?}", err_buf.buf);
println!("{:?}", res);

let (tx, rx) = oneshot::channel::<NativeEvent>();
let tx = Box::into_raw(Box::new(tx)) as *mut c_void;
unsafe { rdkafka_sys::rd_kafka_AdminOptions_set_opaque(native_opts, tx) };

let native_queue = unsafe { rdkafka_sys::rd_kafka_queue_new(client_ptr) };

unsafe {
    rd_kafka_ListConsumerGroups(client_ptr, native_opts, native_queue);
}

let f = ListConsumerGroupsFuture { rx };

let result = executor::block_on(f);

match result {
    Ok(_) => "Success",
    Err(_) => "Failure",
}; */
