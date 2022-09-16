pub fn run() {
    // Rust のスタック上限は8MByte
    // Safe
    let _a1: [u8; 7000000] = [1; 7000000];

    // StackOverFlowが発生する。cargo 1.63.0 (fd9c4297c 2022-07-01)ではsegmentation faultが発生する。
    //let _a2: [u8; 8000000] = [1; 8000000];
}
