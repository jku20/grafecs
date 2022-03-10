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
    | Move { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Rotate { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Apply { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Display { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
    | Save { Ok( Expr::Function { span: $span, typ: Box::new($1?) } ) }
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

Num -> Result<Expr, ()>:
    'NUM' { Ok( Expr::Num { span: $span } ) }
    ;

Axis -> Result<Expr, ()>:
    'AXIS' { Ok( Expr::Axis { span: $span } ) }
    ;

File -> Result<Expr, ()>:
    'FILE' { Ok( Expr::File { span: $span } ) }
    ;

Unmatched -> ():
    'UNMATCHED' { }
    ;
%%

use lrpar::Span;

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
