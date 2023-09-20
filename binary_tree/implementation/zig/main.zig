const std = @import("std");

const Node = struct {
    const Self = @This();

    data: i32 = 0,
    left: ?*Node = null,
    right: ?*Node = null,

    pub fn new(self: *Self, data: i32) void {
        self.data = data;
    }

    pub fn new_with_lr(self: *Self, data: i32, right: ?*Node, left: ?*Node) void {
        new(data);
        self.right = right;
        self.left = left;
    }
};

pub fn main() void {
    // cont print = std.debug.print();
    std.debug.print("hello world", .{});

    // var node0: *Node = Node.new(5);
    var node0: Node = undefined;
    Node.new(&node0, 5);
    var node5 = Node{
        .data = 0,
        .left = null,
        .right = null,
    };
    std.debug.print("the node {}", .{node5});
    // std.debug.print("the node {}", .{node0});
    var node7 = Node.new(&node7, 5);
    std.debug.print("the node {}", .{node7});
}
