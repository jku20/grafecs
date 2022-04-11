%start Expr
%%
Expr -> Result<Expr, ()>:
    Expr 'SPACE' Command
    { 
        match $1? {
            Expr::Expr { span, mut cmds } => {
                cmds.push($3?);
                Ok( Expr::Expr { span, cmds } )
            }
            //if we get here bad bad very bad
            _ => Err(())
        }
    }
    | Command { Ok( Expr::Expr { span: $span, cmds: vec![$1?] } ) }
    ;

Command -> Result<Expr, ()>:
    Function { Ok( Expr::Command { span: $span, fun: Box::new($1?) } ) }
    ;

Function -> Result<Expr, ()>:
    Line { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Ident { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Scale { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Move { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) } | Rotate { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Apply { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Display { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Save { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Circle { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Hermite { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Bezier { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Comment { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Box { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Sphere { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Torus { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Clear { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Push { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Pop { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    ;

Line -> Result<Expr, ()>:
    'LINE' 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num 
    {
        Ok( Expr::Line { span: $span, args: [
            Box::new($3?), Box::new($5?), Box::new($7?), Box::new($9?), Box::new($11?), Box::new($13?)
            ]
        })
    }
    ;

Ident -> Result<Expr, ()>:
    'IDENT' { Ok( Expr::Ident { span: $span } ) }
    ;

Scale -> Result<Expr, ()>:
    'SCALE' 'SPACE' Num 'SPACE' Num 'SPACE' Num
    {
        Ok( Expr::Scale { span: $span, args: [Box::new($3?), Box::new($5?), Box::new($7?)] } )
    }
    ;
        
Move -> Result<Expr, ()>:
    'MOVE' 'SPACE' Num 'SPACE' Num 'SPACE' Num
    {
        Ok( Expr::Move { span: $span, args: [Box::new($3?), Box::new($5?), Box::new($7?)] } )
    }
    ;

Rotate -> Result<Expr, ()>:
    'ROTATE' 'SPACE' Axis 'SPACE' Num
    {
        Ok( Expr::Rotate { span: $span, axis: Box::new($3?), deg: Box::new($5?) } )
    }
    ;

Apply -> Result<Expr, ()>:
    'APPLY' { Ok( Expr::Apply { span: $span } ) }
    ;

Display -> Result<Expr, ()>:
    'DISPLAY' { Ok( Expr::Display { span: $span } ) }
    ;

Save -> Result<Expr, ()>:
    'SAVE' 'SPACE' File { Ok( Expr::Save { span: $span, file: Box::new($3?) } ) }
    ;

Circle -> Result<Expr, ()>:
    'CIRCLE' 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num
    {
        Ok( Expr::Circle { span: $span, cx: Box::new($3?), cy: Box::new($5?), cz: Box::new($7?), r: Box::new($9?) } )
    }
    ;

Hermite -> Result<Expr, ()>:
    'HERMITE' 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num 
    {
        Ok( Expr::Hermite {
            span: $span,
            x0: Box::new($3?),
            y0: Box::new($5?),
            x1: Box::new($7?),
            y1: Box::new($9?),
            rx0: Box::new($11?),
            ry0: Box::new($13?),
            rx1: Box::new($15?),
            ry1: Box::new($17?),
        } )
    }
    ;

Bezier -> Result<Expr, ()>:
    'BEZIER' 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num 
    {
        Ok( Expr::Bezier {
            span: $span,
            x0: Box::new($3?),
            y0: Box::new($5?),
            x1: Box::new($7?),
            y1: Box::new($9?),
            x2: Box::new($11?),
            y2: Box::new($13?),
            x3: Box::new($15?),
            y3: Box::new($17?),
        } )
    }
    ;

Box -> Result<Expr, ()>:
    'BOX' 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num 
    {
        Ok( Expr::Box { span: $span, args: [
            Box::new($3?), Box::new($5?), Box::new($7?), Box::new($9?), Box::new($11?), Box::new($13?)
            ]
        })
    }
    ;

Sphere -> Result<Expr, ()>:
    'SPHERE' 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num
    {
        Ok( Expr::Sphere { span: $span, args: [
            Box::new($3?), Box::new($5?), Box::new($7?), Box::new($9?)
            ]
        })
    }
    ;

Torus -> Result<Expr, ()>:
    'TORUS' 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num 'SPACE' Num
    {
        Ok( Expr::Torus { span: $span, args: [
            Box::new($3?), Box::new($5?), Box::new($7?), Box::new($9?), Box::new($11?)
            ]
        })
    }
    ;

Clear -> Result<Expr, ()>:
    'CLEAR' { Ok( Expr::Clear { span: $span } ) }
    ;

Push -> Result<Expr, ()>:
    'PUSH' { Ok( Expr::Push { span: $span } ) }
    ;

Pop -> Result<Expr, ()>:
    'POP' { Ok( Expr::Pop { span: $span } ) }
    ;

Num -> Result<Expr, ()>:
    'NUM' { Ok( Expr::Num { span: $span } ) }
    ;

Axis -> Result<Expr, ()>:
    'AXIS' { Ok( Expr::Axis { span: $span } ) }
    ;

File -> Result<Expr, ()>:
    'FILE' { Ok( Expr::File { span: $span } ) }
    ;

Comment -> Result<Expr, ()>:
    'COMMENT' { Ok( Expr::Expr { span: $span, cmds: Vec::new() } ) }
    ;

Unmatched -> ():
    'UNMATCHED' { }
    ;
%%

use lrpar::Span;

//FIXME: COMMENTS ONLY WORK FOR REGULARISH STRINGS
///enum specifying all the commands
#[derive(Debug)]
pub enum Expr {
    Expr {
        span: Span,
        cmds: Vec<Expr>,
    },
    Command {
        span: Span,
        fun: Box<Expr>,
    },
    Function {
        span: Span,
        typ: Box<Expr>,
    },
    Line {
        span: Span,
        args: [Box<Expr>; 6],
    },
    Ident {
        span: Span,
    },
    Scale {
        span: Span,
        args: [Box<Expr>; 3],
    },
    Move {
        span: Span,
        args: [Box<Expr>; 3],
    },
    Rotate {
        span: Span,
        axis: Box<Expr>,
        deg: Box<Expr>,
    },
    Apply {
        span: Span,
    },
    Display {
        span: Span,
    },
    Save {
        span: Span,
        file: Box<Expr>,
    },
    Circle {
        span: Span,
        cx: Box<Expr>,
        cy: Box<Expr>,
        cz: Box<Expr>,
        r: Box<Expr>,
    },
    Hermite {
        span: Span,
        x0: Box<Expr>,
        y0: Box<Expr>,
        x1: Box<Expr>,
        y1: Box<Expr>,
        rx0: Box<Expr>,
        ry0: Box<Expr>,
        rx1: Box<Expr>,
        ry1: Box<Expr>,
    },
    Bezier {
        span: Span,
        x0: Box<Expr>,
        y0: Box<Expr>,
        x1: Box<Expr>,
        y1: Box<Expr>,
        x2: Box<Expr>,
        y2: Box<Expr>,
        x3: Box<Expr>,
        y3: Box<Expr>,
    },
    Box {
        span : Span,
        args: [Box<Expr>; 6],
    },
    Sphere {
        span: Span,
        args: [Box<Expr>; 4],
    },
    Torus {
        span: Span,
        args: [Box<Expr>; 5],
    },
    Clear {
        span: Span,
    },
    Push {
        span: Span,
    },
    Pop {
        span: Span,
    },
    Num {
        span: Span,
    },
    Axis {
        span: Span,
    },
    File {
        span: Span,
    },
}
