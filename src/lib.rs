#[cfg(test)]
mod tests {
    extern crate graphviz_rust;

    use std::detect::__is_feature_detected::sha;
    use std::fs;
    use graphviz_rust::dot_structures::*;
    use graphviz_rust::dot_generator::*;
    use graphviz_rust::parse;
    use graphviz_rust::attributes::GraphAttributes as GAttributes;
    use self::graphviz_rust::attributes::{EdgeAttributes, NodeAttributes, rankdir, shape};
    use self::graphviz_rust::cmd::{CommandArg, Format};
    use self::graphviz_rust::exec;
    use self::graphviz_rust::printer::{DotPrinter, PrinterContext};

    #[test]
    fn parse_test() {
        let dot_graph =
            r#"
                 graph G {
                    layout=neato
                    run -- intr;
                    intr -- runbl;
                    runbl -- run;
                    run -- kernel;
                    kernel -- zombie;
                    kernel -- sleep;
                    kernel -- runmem;
                    sleep -- swap;
                    swap -- runswap;
                    runswap -- new;
                    runswap -- runmem;
                    new -- runmem;
                    sleep -- runmem;
                }
            "#;

        let graph: Result<Graph, String> = parse(dot_graph);
        assert!(graph.is_ok())
    }

    #[test]
    fn create_test() {
        let dot_graph =
            r#"
                 graph G {
                    layout=neato
                    run -- intr;
                    intr -- runbl;
                    runbl -- run;
                    run -- kernel;
                    kernel -- zombie;
                    kernel -- sleep;
                    kernel -- runmem;
                    sleep -- swap;
                    swap -- runswap;
                    runswap -- new;
                    runswap -- runmem;
                    new -- runmem;
                    sleep -- runmem;
                }
            "#;

        let graph: Result<Graph, String> = parse(dot_graph);
        assert!(graph.is_ok());
        let manual_graph = graph!(id!("G");
            attr!("layout","neato"),
            edge!(node_id!("run") => node_id!("intr")),
            edge!(node_id!("intr") => node_id!("runbl")),
            edge!(node_id!("runbl") => node_id!("run")),
            edge!(node_id!("run") => node_id!("kernel")),
            edge!(node_id!("kernel") => node_id!("zombie")),
            edge!(node_id!("kernel") => node_id!("sleep")),
            edge!(node_id!("kernel") => node_id!("runmem")),
            edge!(node_id!("sleep") => node_id!("swap")),
            edge!(node_id!("swap") => node_id!("runswap")),
            edge!(node_id!("runswap") => node_id!("new")),
            edge!(node_id!("runswap") => node_id!("runmem")),
            edge!(node_id!("new") => node_id!("runmem")),
            edge!(node_id!("sleep") => node_id!("runmem"))
        );

        assert_eq!(graph.unwrap(), manual_graph);
    }

    #[test]
    fn attribute_test() {
        let graph_str =
            r#"
                digraph finite_state_machine {
                    rankdir=LR;
                    size=8.5
                    node [shape = doublecircle]; 0 3 4 8;
                    node [shape = circle];
                    0 -> 2 [label = "SS(B)"];
                    0 -> 1 [label = "SS(S)"];
                    1 -> 3 [label = "S($end)"];
                    2 -> 6 [label = "SS(b)"];
                    2 -> 5 [label = "SS(a)"];
                    2 -> 4 [label = "S(A)"];
                    5 -> 7 [label = "S(b)"];
                    5 -> 5 [label = "S(a)"];
                    6 -> 6 [label = "S(b)"];
                    6 -> 5 [label = "S(a)"];
                    7 -> 8 [label = "S(b)"];
                    7 -> 5 [label = "S(a)"];
                    8 -> 6 [label = "S(b)"];
                    8 -> 5 [label = "S(a)"];
                }
            "#;
        let graph: Result<Graph, String> = parse(graph_str);
        assert!(graph.is_ok());

        let manual_graph =
            graph!(di id!("finite_state_machine");
                GAttributes::rankdir(rankdir::LR),
                GAttributes::size(8.5),
                GraphAttributes::Node(vec![NodeAttributes::shape(shape::doublecircle)]),
                node!("0"),node!("3"),node!("4"),node!("8"),
                GraphAttributes::Node(vec![NodeAttributes::shape(shape::circle)]),
                edge!(node_id!("0") => node_id!("2");attr!("label",esc "\"SS(B)\"")),
                edge!(node_id!("0") => node_id!("1");attr!("label",esc "\"SS(S)\"")),
                edge!(node_id!("1") => node_id!("3");attr!("label",esc "\"S($end)\"")),
                edge!(node_id!("2") => node_id!("6");attr!("label",esc "\"SS(b)\"")),
                edge!(node_id!("2") => node_id!("5");attr!("label",esc "\"SS(a)\"")),
                edge!(node_id!("2") => node_id!("4");attr!("label",esc "\"S(A)\"")),
                edge!(node_id!("5") => node_id!("7");attr!("label",esc "\"S(b)\"")),
                edge!(node_id!("5") => node_id!("5");attr!("label",esc "\"S(a)\"")),
                edge!(node_id!("6") => node_id!("6");attr!("label",esc "\"S(b)\"")),
                edge!(node_id!("6") => node_id!("5");attr!("label",esc "\"S(a)\"")),
                edge!(node_id!("7") => node_id!("8");attr!("label",esc "\"S(b)\"")),
                edge!(node_id!("7") => node_id!("5");attr!("label",esc "\"S(a)\"")),
                edge!(node_id!("8") => node_id!("6");attr!("label",esc "\"S(b)\"")),
                edge!(node_id!("8") => node_id!("5");attr!("label",esc "\"S(a)\""))
            );

        assert_eq!(graph.unwrap(), manual_graph);
    }

    fn print_test(){
        let mut g = graph!(id!("id"));

        for el in (1..10).into_iter() {
            if el % 2 == 0 {
                g.add_stmt(stmt!(node!(el)))
            } else {
                g.add_stmt(stmt!(subgraph!(el)))
            }
        }
        println!("{}",g.print(&mut PrinterContext::default()));
        assert_eq!(178896, g.print(&mut PrinterContext::default()).len())
    }

    #[test]
    fn exec_test() {
        let mut g = graph!(id!("id");
            node!("nod"),
            subgraph!("sb";
                edge!(node_id!("a") => subgraph!(;
                    node!("n";
                    NodeAttributes::color(color_name::black), NodeAttributes::shape(shape::egg))
                ))
            ),
            edge!(node_id!("a1") => node_id!(esc "a2"))
        );


        let p = "1.svg";
        let out = exec(g.clone(), PrinterContext::default(), vec![
            CommandArg::Format(Format::Svg),
            CommandArg::Output(p.to_string())
        ]).unwrap();

        assert_eq!("",out);

    }
}
