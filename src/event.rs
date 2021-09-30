use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type NodeId = String;
pub type EdgeId = String;
pub type PropName = String;
pub type PropValue = String;
//pub type Coord = u32;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Event {
    // Is this the format if we have more than 100 events?
    AddNode {
        id: Option<NodeId>,
        props: HashMap<PropName, PropValue>,
    },
    AddEdge {
        id: Option<NodeId>,
        from: EdgeId,
        to: NodeId,
        props: HashMap<PropName, PropValue>,
    },
    SetNodeProperties {
        id: NodeId,
        props: HashMap<PropName, Option<PropValue>>,
    },
    SetEdgeProperties {
        id: EdgeId,
        from: Option<NodeId>,
        to: Option<NodeId>,
        props: HashMap<PropName, Option<PropValue>>,
    },
    RemoveNode {
        id: NodeId,
    },
    RemoveEdge {
        id: EdgeId,
    },
}

use nom::{bytes::complete::*, character::complete::*, combinator::*, sequence::*};

/*
impl Event {
    pub fn parse(mut input: &str) -> (&str, Vec<Event>) {
        many0(parse_event)(input);

        // == OR

        let mut events = Vec::new();
        loop {
            (new_input, event) = parse_event(input);
            input = new_input;
            match event {
                Ok(ok) => {
                    events.push(event);
                }
                Err(err) => {
                    return (input, events);
                }
            }
        }
    }

    fn parse_event(input: &str) -> (&str, Event) {
        alt((parse_add, parse_set, parse_print, parse_remove))(input)
    }

    fn parse_add(input: &str) -> (&str, Event) {
        preceded(pair("add", is_a(" ")), opt(parse_opt_id))(input)
    }

    fn parse_opt_id(input: &str) -> (&str, Event) {
        preceded("id=", terminated(is_not(" "), " "))(input)
    }
}
*/
/*


add edge

add edge id=qwe from=asd to=zxc
add edge qwe asd zxc
add edge asd zxc

add node id=asd props.a=123 props.b=123 props.c=123 props.d=123
add node id=asd props: a=123 b=123 c=123 d=123
add node *asd props.a=123
add type=node asd props.a=123

set node id=asd props.a="qwdasd\" qwaweqwe"
set node asd props.a="qwdasd\" qwaweqwe"

set node props a="123" b="123" c= d=

print node asd props.a
print node asd

remove node id=asd
remove node asd
remove edge asd



Other considerations
create a command generator
return last command
return last created node/edge

command enum{
    success //command worked
    cancel //user canceled command
    nothing /command did nothing and cancel was not pressed
    failure //command failed, bad input, bad computation
    unknown_command // not found or typo
    exit_app //app exited
}

is_repeatable // bool, can be repeated by pressing Enter immediately after the command finishes
is_running
is_undoable //make all commands undoable
command help
get_command_names
lookup_command(by uuid or name)
clear_line


node_t1
    name amir
    color red
    timestamp 12:00

node_t2
    name Amir
    color red
    timestamp 12.05

node_t3
    name Am
    color red
    timestamp 12.06

node_t4
    name Am
    timestamp 12.10


enum Change {
    NodeChanged {
        timestamp: TimeStamp
        id: NodeId,
        prop: PropernName,
        prev: Option<PropValue>,
        next: Option<PropValue>,
    }
    NodeRemove {
        id:
        poprs: HashMap<PropValue>,
    }
    EdgeChanged {
        timestamp: TimeStamp
        id: EdgeId,
        prop: PropernName,
        prev: Option<PropValue>,
        next: Option<PropValue>,
    }
}

change[0]
change[1]
change[2]
change[3]
change[4]




add_node id=asd
set_props id=asd


*/
