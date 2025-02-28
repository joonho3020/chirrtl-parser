mod lexer;


#[cfg(test)]
mod test {
    use crate::lexer::*;
    use logos::Logos;

    #[test]
    fn gcd() {
        let source =
r#"FIRRTL version 3.3.0
circuit GCD :
  module GCD : @[src/main/scala/gcd/GCD.scala 15:7]
    input clock : Clock @[src/main/scala/gcd/GCD.scala 15:7]
    input reset : UInt<1> @[src/main/scala/gcd/GCD.scala 15:7]
    output io : { flip value1 : UInt<16>, flip value2 : UInt<16>, flip loadingValues : UInt<1>, outputGCD : UInt<16>, outputValid : UInt<1>} @[src/main/scala/gcd/GCD.scala 16:14]

    reg x : UInt, clock @[src/main/scala/gcd/GCD.scala 24:15]
    reg y : UInt, clock @[src/main/scala/gcd/GCD.scala 25:15]
    node _T = gt(x, y) @[src/main/scala/gcd/GCD.scala 27:10]
    when _T : @[src/main/scala/gcd/GCD.scala 27:15]
      node _x_T = sub(x, y) @[src/main/scala/gcd/GCD.scala 27:24]
      node _x_T_1 = tail(_x_T, 1) @[src/main/scala/gcd/GCD.scala 27:24]
      connect x, _x_T_1 @[src/main/scala/gcd/GCD.scala 27:19]
    else :
      node _y_T = sub(y, x) @[src/main/scala/gcd/GCD.scala 28:25]
      node _y_T_1 = tail(_y_T, 1) @[src/main/scala/gcd/GCD.scala 28:25]
      connect y, _y_T_1 @[src/main/scala/gcd/GCD.scala 28:20]
    when io.loadingValues : @[src/main/scala/gcd/GCD.scala 30:26]
      connect x, io.value1 @[src/main/scala/gcd/GCD.scala 31:7]
      connect y, io.value2 @[src/main/scala/gcd/GCD.scala 32:7]
    connect io.outputGCD, x @[src/main/scala/gcd/GCD.scala 35:16]
    node _io_outputValid_T = eq(y, UInt<1>(0h0)) @[src/main/scala/gcd/GCD.scala 36:23]
    connect io.outputValid, _io_outputValid_T @[src/main/scala/gcd/GCD.scala 36:18]
"#;

        let mut lex = FIRRTLLexer::new(source);
        while let Some(ts) = lex.next() {
            println!("- {:?}", ts);
            match ts.token {
                Token::Error => {
                    panic!("Got a error token");
                }
                _ => { }
            }
        }
    }

    #[test]
    fn one_read_one_write_sram() {
        let source =
r#"FIRRTL version 3.3.0
circuit OneReadOneWritePortSRAM :
  module OneReadOneWritePortSRAM : @[src/main/scala/gcd/SRAM.scala 10:7]
    input clock : Clock @[src/main/scala/gcd/SRAM.scala 10:7]
    input reset : UInt<1> @[src/main/scala/gcd/SRAM.scala 10:7]
    output io : { flip ren : UInt<1>, flip raddr : UInt<3>, rdata : UInt<2>[4], flip wen : UInt<1>, flip waddr : UInt<3>, flip wdata : UInt<2>[4], flip wmask : UInt<1>[4]} @[src/main/scala/gcd/SRAM.scala 11:14]

    smem mem : UInt<2>[4] [8] @[src/main/scala/gcd/SRAM.scala 22:24]
    when io.wen : @[src/main/scala/gcd/SRAM.scala 23:17]
      write mport MPORT = mem[io.waddr], clock @[src/main/scala/gcd/SRAM.scala 24:14]
      when io.wmask[0] : @[src/main/scala/gcd/SRAM.scala 24:14]
        connect MPORT[0], io.wdata[0] @[src/main/scala/gcd/SRAM.scala 24:14]
      when io.wmask[1] : @[src/main/scala/gcd/SRAM.scala 24:14]
        connect MPORT[1], io.wdata[1] @[src/main/scala/gcd/SRAM.scala 24:14]
      when io.wmask[2] : @[src/main/scala/gcd/SRAM.scala 24:14]
        connect MPORT[2], io.wdata[2] @[src/main/scala/gcd/SRAM.scala 24:14]
      when io.wmask[3] : @[src/main/scala/gcd/SRAM.scala 24:14]
        connect MPORT[3], io.wdata[3] @[src/main/scala/gcd/SRAM.scala 24:14]
    wire _WIRE : UInt<3> @[src/main/scala/gcd/SRAM.scala 26:23]
    invalidate _WIRE @[src/main/scala/gcd/SRAM.scala 26:23]
    when io.ren : @[src/main/scala/gcd/SRAM.scala 26:23]
      connect _WIRE, io.raddr @[src/main/scala/gcd/SRAM.scala 26:23]
      read mport MPORT_1 = mem[_WIRE], clock @[src/main/scala/gcd/SRAM.scala 26:23]
    connect io.rdata, MPORT_1 @[src/main/scala/gcd/SRAM.scala 26:12]
    "#;

        let mut lex = FIRRTLLexer::new(source);
        while let Some(ts) = lex.next() {
            println!("- {:?}", ts);
            match ts.token {
                Token::Error => {
                    panic!("Got a error token");
                }
                _ => { }
            }
        }
    }
}
