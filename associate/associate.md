#associate

##类型族

    trait Graph<N, E> {
        fn has_edge(&self, &N, &N) -> bool;
        fn edges(&self, &N) -> Vec<E>;
        // etc
    }
    
    fn distance<N, E, G: Graph<N, E>>(graph: &G, start: &N, end: &N) -> u32 { ... }


==》

    trait Graph {
        type N;
        type E;

        fn has_edge(&self, &Self::N, &Self::N) -> bool;
        fn edges(&self, &Self::N) -> Vec<Self::E>;
        // etc
    }
    
    fn distance<G: Graph>(graph: &G, start: &G::N, end: &G::N) -> uint { ... }


##实现

    # trait Graph {
    #     type N;
    #     type E;
    #     fn has_edge(&self, &Self::N, &Self::N) -> bool;
    #     fn edges(&self, &Self::N) -> Vec<Self::E>;
    # }
    struct Node;

    struct Edge;

    struct MyGraph;

    impl Graph for MyGraph {
        type N = Node;
        type E = Edge;

        fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
            true
        }

        fn edges(&self, n: &Node) -> Vec<Edge> {
            Vec::new()
        }
    }