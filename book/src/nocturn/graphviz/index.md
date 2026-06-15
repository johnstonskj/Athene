<!-- markdownlint-disable-file MD033 -->
# Appendix: GraphViz

## Nodes

Default node configuration.

```dot
node [fontname="Helvetica", fontcolor="black", color="#808080"];
```

![Nodes](./dot-nodes.svg)

### GraphViz Rendering

![Nodes in GraphViz](./example-nodes.svg)

<details>
  <summary>Show dot source</summary>

```dot
strict digraph {
  rankdir="BT"

  node [fontname="Helvetica", fontcolor="black", color="#808080"];
  edge [fontname="Helvetica", fontcolor="black", color="#808080"];


  subgraph cluster_01 {
    rankdir="LR";
    peripheries = 0;
      
    Class [shape="rect"];
       
    Ontology [ shape="tab"];

    Datatype [shape="rect", style="dotted"];
  };
   

  subgraph cluster_02 {
    rankdir="LR";
    peripheries = 0;
      
    Individual [shape="ellipse"];
 
    axioms [ shape="note"];

    Literal [shape="ellipse", style="dotted"];
  };

  subgraph cluster_03 {
    rankdir="LR";
    peripheries = 0;
      
    OProperty  [shape="polygon", sides=3, orientation=270];
    
    DProperty  [shape="polygon", sides=3, orientation=270, style="dotted"];

    AProperty [shape="house", orientation=270];
  };

  axioms -> Ontology [style="invisible", dir="none"];
  DProperty -> axioms [style="invisible", dir="none"];
}
```

</details>

### Literals

![Literals in GraphViz](./example-literals.svg)

<details>
  <summary>Show dot source</summary>

```dot
strict digraph {
  rankdir="LR"

  node [fontname="Helvetica", fontcolor="black", color="#808080"];
  edge [fontname="Helvetica", fontcolor="black", color="#808080"];


  Literal [shape="ellipse", style="dotted", label="?"];

  LanguageString [shape="ellipse", style="dotted", label=<
    <TABLE BORDER="0" CELLBORDER="0" CELLPADDING="0" CELLSPACING="0">
        <TR>
            <TD>Simon</TD>
        </TR>
        <HR/>
        <TR>
            <TD>@en</TD>
        </TR>
    </TABLE>
  >];
  
  TypedLiteral [shape="ellipse", style="dotted", label=<
    <TABLE BORDER="0" CELLBORDER="0" CELLPADDING="0" CELLSPACING="0">
        <TR>
            <TD>21</TD>
        </TR>
        <HR/>
        <TR>
            <TD>xsd:integer</TD>
        </TR>
    </TABLE>
  >];

  Literal -> LanguageString [style="invisible", dir="none"];
  LanguageString -> TypedLiteral [style="invisible", dir="none"];
}
```

</details>

## Edges
