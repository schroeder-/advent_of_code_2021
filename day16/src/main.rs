fn hex_to_bin(h: char) -> &'static str {
    match h {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

#[derive(Debug, PartialEq)]
enum PaketOperator {
    Sum,
    Product,
    Minimum,
    Maximum,
    Gtr,
    Lt,
    Eq,
    Value,
    Zero,
}

impl PaketOperator {
    fn from_type(t: usize) -> Self {
        match t {
            0 => PaketOperator::Sum,
            1 => PaketOperator::Product,
            2 => PaketOperator::Minimum,
            3 => PaketOperator::Maximum,
            4 => PaketOperator::Value,
            5 => PaketOperator::Gtr,
            6 => PaketOperator::Lt,
            7 => PaketOperator::Eq,
            _ => PaketOperator::Zero,
        }
    }
}
#[derive(Debug, PartialEq)]
struct Operator {
    operator: PaketOperator,
    ops: Vec<Paket>,
}

#[derive(Debug, PartialEq)]
enum PaketContent {
    Value(usize),
    Operator(Operator),
}

#[derive(Debug, PartialEq)]
struct Paket {
    version: usize,
    content: PaketContent,
}

impl Paket {
    fn parse_part(bin_str: &mut String) -> usize {
        let mut r = "".to_owned();
        loop {
            if bin_str.is_empty() {
                break;
            }
            let stop = bin_str.drain(0..1).next().unwrap_or_default() == '0';
            r.extend(bin_str.drain(0..=3));
            if stop {
                break;
            }
        }
        usize::from_str_radix(&r, 2).unwrap_or_default()
    }

    fn from_drain_str(bin_str: &mut String) -> Self {
        let version = usize::from_str_radix(bin_str.drain(0..3).as_str(), 2).unwrap_or_default();
        let operator = PaketOperator::from_type(
            usize::from_str_radix(bin_str.drain(0..3).as_str(), 2).unwrap_or_default(),
        );
        let content = if operator == PaketOperator::Value {
            PaketContent::Value(Self::parse_part(bin_str))
        } else {
            let l = if bin_str.drain(0..1).next().unwrap_or_default() == '1' {
                11
            } else {
                15
            };
            let len = usize::from_str_radix(bin_str.drain(0..l).as_str(), 2).unwrap_or_default();
            if l == 11 {
                PaketContent::Operator(Operator {
                    operator,
                    ops: (0..len).map(|_| Self::from_drain_str(bin_str)).collect(),
                })
            } else {
                let mut data: String = bin_str.drain(..len).collect();
                let mut res: Vec<Self> = Vec::new();
                while data.len() > 5 {
                    res.push(Self::from_drain_str(&mut data));
                }
                PaketContent::Operator(Operator { operator, ops: res })
            }
        };
        Self { version, content }
    }

    fn from_hex_str(inp: &str) -> Self {
        let mut s: String = inp.chars().map(hex_to_bin).collect();
        Self::from_drain_str(&mut s)
    }

    fn get_version_sum(&self) -> usize {
        let sub = if let PaketContent::Operator(o) = &self.content {
            o.ops.iter().map(Self::get_version_sum).sum()
        } else {
            0
        };
        self.version + sub
    }

    fn get_result(&self) -> usize {
        match &self.content {
            PaketContent::Value(v) => *v,
            PaketContent::Operator(o) => match o.operator {
                PaketOperator::Sum => o.ops.iter().map(Self::get_result).sum(),
                PaketOperator::Product => o.ops.iter().map(Self::get_result).product(),
                PaketOperator::Minimum => {
                    o.ops.iter().map(Self::get_result).min().unwrap_or_default()
                }
                PaketOperator::Maximum => {
                    o.ops.iter().map(Self::get_result).max().unwrap_or_default()
                }
                PaketOperator::Gtr | PaketOperator::Lt | PaketOperator::Eq => {
                    if let [a, b] = &o.ops[..] {
                        let a = a.get_result();
                        let b = b.get_result();
                        match o.operator {
                            PaketOperator::Gtr => {
                                if a > b {
                                    1
                                } else {
                                    0
                                }
                            }
                            PaketOperator::Lt => {
                                if a < b {
                                    1
                                } else {
                                    0
                                }
                            }
                            PaketOperator::Eq => {
                                if a == b {
                                    1
                                } else {
                                    0
                                }
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        0
                    }
                }
                PaketOperator::Zero => 0,
                _ => unreachable!(),
            },
        }
    }
}

#[test]
fn test_parsing() {
    let input = "D2FE28";
    let p1 = Paket::from_hex_str(&input);
    assert_eq!(
        p1,
        Paket {
            version: 6,
            content: PaketContent::Value(2021)
        }
    );
    let input = "38006F45291200";
    let p2 = Paket::from_hex_str(&input);
    assert_eq!(
        p2,
        Paket {
            version: 1,
            content: PaketContent::Operator(Operator {
                operator: PaketOperator::Lt,

                ops: vec![
                    Paket {
                        version: 6,
                        content: PaketContent::Value(10)
                    },
                    Paket {
                        version: 2,
                        content: PaketContent::Value(20)
                    },
                ]
            })
        }
    );
    let input = "EE00D40C823060";
    let p3 = Paket::from_hex_str(&input);
    assert_eq!(
        p3,
        Paket {
            version: 7,
            content: PaketContent::Operator(Operator {
                operator: PaketOperator::Maximum,
                ops: vec![
                    Paket {
                        version: 2,
                        content: PaketContent::Value(1)
                    },
                    Paket {
                        version: 4,
                        content: PaketContent::Value(2)
                    },
                    Paket {
                        version: 1,
                        content: PaketContent::Value(3)
                    }
                ]
            })
        }
    );
}

#[test]
fn test_version_sum() {
    let vs = Paket::from_hex_str("8A004A801A8002F478").get_version_sum();
    assert_eq!(vs, 16);
    let vs = Paket::from_hex_str("620080001611562C8802118E34").get_version_sum();
    assert_eq!(vs, 12);
    let vs = Paket::from_hex_str("C0015000016115A2E0802F182340").get_version_sum();
    assert_eq!(vs, 23);
    let vs = Paket::from_hex_str("A0016C880162017C3686B18A3D4780").get_version_sum();
    assert_eq!(vs, 31);
}

#[test]
fn test_operators() {
    assert_eq!(Paket::from_hex_str("C200B40A82").get_result(), 3);
    assert_eq!(Paket::from_hex_str("04005AC33890").get_result(), 54);
    assert_eq!(Paket::from_hex_str("880086C3E88112").get_result(), 7);
    assert_eq!(Paket::from_hex_str("CE00C43D881120").get_result(), 9);
    assert_eq!(Paket::from_hex_str("D8005AC2A8F0").get_result(), 1);
    assert_eq!(Paket::from_hex_str("F600BC2D8F").get_result(), 0);
    assert_eq!(Paket::from_hex_str("9C005AC2F8F0").get_result(), 0);
    assert_eq!(
        Paket::from_hex_str("9C0141080250320F1802104A08").get_result(),
        1
    );
}

fn main() {
    let input = "2052ED9802D3B9F465E9AE6003E52B8DEE3AF97CA38100957401A88803D05A25C1E00043E1545883B397259385B47E40257CCEDC7401700043E3F42A8AE0008741E8831EC8020099459D40994E996C8F4801CDC3395039CB60E24B583193DD75D299E95ADB3D3004E5FB941A004AE4E69128D240130D80252E6B27991EC8AD90020F22DF2A8F32EA200AC748CAA0064F6EEEA000B948DFBED7FA4660084BCCEAC01000042E37C3E8BA0008446D8751E0C014A0036E69E226C9FFDE2020016A3B454200CBAC01399BEE299337DC52A7E2C2600BF802B274C8848FA02F331D563B3D300566107C0109B4198B5E888200E90021115E31C5120043A31C3E85E400874428D30AA0E3804D32D32EED236459DC6AC86600E4F3B4AAA4C2A10050336373ED536553855301A600B6802B2B994516469EE45467968C016D004E6E9EE7CE656B6D34491D8018E6805E3B01620C053080136CA0060801C6004A801880360300C226007B8018E0073801A801938004E2400E01801E800434FA790097F39E5FB004A5B3CF47F7ED5965B3CF47F7ED59D401694DEB57F7382D3F6A908005ED253B3449CE9E0399649EB19A005E5398E9142396BD1CA56DFB25C8C65A0930056613FC0141006626C5586E200DC26837080C0169D5DC00D5C40188730D616000215192094311007A5E87B26B12FCD5E5087A896402978002111960DC1E0004363942F8880008741A8E10EE4E778FA2F723A2F60089E4F1FE2E4C5B29B0318005982E600AD802F26672368CB1EC044C2E380552229399D93C9D6A813B98D04272D94440093E2CCCFF158B2CCFE8E24017CE002AD2940294A00CD5638726004066362F1B0C0109311F00424CFE4CF4C016C004AE70CA632A33D2513004F003339A86739F5BAD5350CE73EB75A24DD22280055F34A30EA59FE15CC62F9500";
    let p = Paket::from_hex_str(input);
    println!("Version sum: {}", p.get_version_sum());
    println!("Result: {}", p.get_result());
}
