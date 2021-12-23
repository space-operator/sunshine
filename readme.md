



    mod magic {
        pub struct Marker1;
        pub struct Marker2;
        pub struct Marker3;

        pub trait Split2<T1, T2> {
            fn split(self) -> (T1, T2);
        }

        pub trait Split<T1, T2, T3> {
            fn split(self) -> (T1, T2);
        }

        impl<T, T1, T2, T3> Split2<T1, T2> for T
        where
            T: Split<T1, T2, T3>,
        {
            fn split(self) -> (T1, T2) {
                Split::split(self)
            }
        }

        pub struct Container<T1, T2, T3>(T1, T2, T3);

        impl<T1, T2, T3> Split<T1, (T2, T3), Marker1> for Container<T1, T2, T3> {
            fn split(self) -> (T1, (T2, T3)) {
                (self.0, (self.1, self.2))
            }
        }

        impl<T1, T2, T3> Split<T2, (T1, T3), Marker2> for Container<T1, T2, T3> {
            fn split(self) -> (T2, (T1, T3)) {
                (self.1, (self.0, self.2))
            }
        }

        fn test() {
            let c = Container(1, 1.0, true);
            let s: (i32, (f64, bool)) = c.split();

            let c = Container(1, 1.0, true);
            let s: (f64, (i32, bool)) = c.split();
        }
    }



multiline text input

"Andrej is here with me today /command 
and this is /command"


touch, mouse,etc. 






















Rust Integrated Dart

https://thlorenz.com/rid-site/


Flutter side
    Terminal like command line
    Visualizer
    Log similar film editing timeline
        each line is one event



Horizontal timeline
    keyboard         create_node   name    ctrl    c              ctrl  v
    mouse
        right      x <ms x          
        left                               x      xxx              xxx
    


    example1:
        select a node, drag it, hold in above another for longer than 300ms, nest_node()
        mouse
            selected id      id123                 id456 300<x<800ms 
            moving x,y          xxxxxxxxxxxxxxxxxxx
        
    

match them against some pattern

text input field / command
    create_node (in the viz, it will a graph node)
        enter parameter/arg: name: Amir
    
    move_node
        provide node id 
            x: 10
            y: 50

    move the coordinates of a node

    copy


if with mouse
    I double click 
        call the create_node function

if keyboard control +c > copy()


Language/protocol where everything work at command line first, then we map some to the events





1. on Flutter side, I will capture all the events, mouse, keyboard, touch, etc, and send them to Rust side
    Flutter side   input: stream of JSON, mock it


2. Rust side
    assume we have the input
    
    Process the steam
    Parse
    Match to a function 
        eg. double_click()




take the all the character/strings
event when it can't be parsed


/*

  {
    "key": "Alt+d",
    "command": "editor.action.moveSelectionToPreviousFindMatch",
    "when": "editorHasSelection && !findInputFocussed"
  },
  {
    "key": "Ctrl+Alt+d",
    "command": "editor.action.addSelectionToPreviousFindMatch",
  },
  {
    "key": "Alt+f",
    "command": "editor.action.moveSelectionToNextFindMatch",
    "when": "editorHasSelection && !findInputFocussed"
  },
  {
    "key": "Ctrl+Alt+f",
    "command": "editor.action.addSelectionToNextFindMatch",
  },

*/