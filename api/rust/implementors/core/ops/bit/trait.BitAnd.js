(function() {var implementors = {
"ahash":[["impl&lt;T, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;&amp;<a class=\"struct\" href=\"ahash/struct.AHashSet.html\" title=\"struct ahash::AHashSet\">AHashSet</a>&lt;T, S&gt;&gt; for &amp;<a class=\"struct\" href=\"ahash/struct.AHashSet.html\" title=\"struct ahash::AHashSet\">AHashSet</a>&lt;T, S&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,</span>"]],
"fixedbitset":[["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;&amp;'a <a class=\"struct\" href=\"fixedbitset/struct.FixedBitSet.html\" title=\"struct fixedbitset::FixedBitSet\">FixedBitSet</a>&gt; for &amp;'a <a class=\"struct\" href=\"fixedbitset/struct.FixedBitSet.html\" title=\"struct fixedbitset::FixedBitSet\">FixedBitSet</a>"]],
"hashbrown":[["impl&lt;T, S, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;&amp;<a class=\"struct\" href=\"hashbrown/hash_set/struct.HashSet.html\" title=\"struct hashbrown::hash_set::HashSet\">HashSet</a>&lt;T, S, A&gt;&gt; for &amp;<a class=\"struct\" href=\"hashbrown/hash_set/struct.HashSet.html\" title=\"struct hashbrown::hash_set::HashSet\">HashSet</a>&lt;T, S, A&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,\n    A: Allocator,</span>"]],
"indexmap":[["impl&lt;T, S1, S2&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;&amp;<a class=\"struct\" href=\"indexmap/set/struct.IndexSet.html\" title=\"struct indexmap::set::IndexSet\">IndexSet</a>&lt;T, S2&gt;&gt; for &amp;<a class=\"struct\" href=\"indexmap/set/struct.IndexSet.html\" title=\"struct indexmap::set::IndexSet\">IndexSet</a>&lt;T, S1&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/hash/trait.Hash.html\" title=\"trait core::hash::Hash\">Hash</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>,\n    S1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,\n    S2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a>,</span>"]],
"rustix":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"rustix/io/struct.FdFlags.html\" title=\"struct rustix::io::FdFlags\">FdFlags</a>&gt; for <a class=\"struct\" href=\"rustix/io/struct.FdFlags.html\" title=\"struct rustix::io::FdFlags\">FdFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"rustix/io/struct.DupFlags.html\" title=\"struct rustix::io::DupFlags\">DupFlags</a>&gt; for <a class=\"struct\" href=\"rustix/io/struct.DupFlags.html\" title=\"struct rustix::io::DupFlags\">DupFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"rustix/io/struct.ReadWriteFlags.html\" title=\"struct rustix::io::ReadWriteFlags\">ReadWriteFlags</a>&gt; for <a class=\"struct\" href=\"rustix/io/struct.ReadWriteFlags.html\" title=\"struct rustix::io::ReadWriteFlags\">ReadWriteFlags</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"rustix/termios/struct.LocalModes.html\" title=\"struct rustix::termios::LocalModes\">LocalModes</a>&gt; for <a class=\"struct\" href=\"rustix/termios/struct.LocalModes.html\" title=\"struct rustix::termios::LocalModes\">LocalModes</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"rustix/termios/struct.OutputModes.html\" title=\"struct rustix::termios::OutputModes\">OutputModes</a>&gt; for <a class=\"struct\" href=\"rustix/termios/struct.OutputModes.html\" title=\"struct rustix::termios::OutputModes\">OutputModes</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"rustix/termios/struct.ControlModes.html\" title=\"struct rustix::termios::ControlModes\">ControlModes</a>&gt; for <a class=\"struct\" href=\"rustix/termios/struct.ControlModes.html\" title=\"struct rustix::termios::ControlModes\">ControlModes</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"rustix/termios/struct.InputModes.html\" title=\"struct rustix::termios::InputModes\">InputModes</a>&gt; for <a class=\"struct\" href=\"rustix/termios/struct.InputModes.html\" title=\"struct rustix::termios::InputModes\">InputModes</a>"]],
"tokio":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"tokio/io/struct.Ready.html\" title=\"struct tokio::io::Ready\">Ready</a>&gt; for <a class=\"struct\" href=\"tokio/io/struct.Ready.html\" title=\"struct tokio::io::Ready\">Ready</a>"]],
"typenum":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"typenum/bit/struct.B0.html\" title=\"struct typenum::bit::B0\">B0</a>&gt; for <a class=\"struct\" href=\"typenum/bit/struct.B1.html\" title=\"struct typenum::bit::B1\">B1</a>"],["impl&lt;Rhs: <a class=\"trait\" href=\"typenum/marker_traits/trait.Bit.html\" title=\"trait typenum::marker_traits::Bit\">Bit</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;Rhs&gt; for <a class=\"struct\" href=\"typenum/bit/struct.B0.html\" title=\"struct typenum::bit::B0\">B0</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;<a class=\"struct\" href=\"typenum/bit/struct.B1.html\" title=\"struct typenum::bit::B1\">B1</a>&gt; for <a class=\"struct\" href=\"typenum/bit/struct.B1.html\" title=\"struct typenum::bit::B1\">B1</a>"],["impl&lt;Ur: <a class=\"trait\" href=\"typenum/marker_traits/trait.Unsigned.html\" title=\"trait typenum::marker_traits::Unsigned\">Unsigned</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;Ur&gt; for <a class=\"struct\" href=\"typenum/uint/struct.UTerm.html\" title=\"struct typenum::uint::UTerm\">UTerm</a>"],["impl&lt;Ul: <a class=\"trait\" href=\"typenum/marker_traits/trait.Unsigned.html\" title=\"trait typenum::marker_traits::Unsigned\">Unsigned</a>, Bl: <a class=\"trait\" href=\"typenum/marker_traits/trait.Bit.html\" title=\"trait typenum::marker_traits::Bit\">Bit</a>, Ur: <a class=\"trait\" href=\"typenum/marker_traits/trait.Unsigned.html\" title=\"trait typenum::marker_traits::Unsigned\">Unsigned</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.74.0/core/ops/bit/trait.BitAnd.html\" title=\"trait core::ops::bit::BitAnd\">BitAnd</a>&lt;Ur&gt; for <a class=\"struct\" href=\"typenum/uint/struct.UInt.html\" title=\"struct typenum::uint::UInt\">UInt</a>&lt;Ul, Bl&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"typenum/uint/struct.UInt.html\" title=\"struct typenum::uint::UInt\">UInt</a>&lt;Ul, Bl&gt;: PrivateAnd&lt;Ur&gt;,\n    PrivateAndOut&lt;<a class=\"struct\" href=\"typenum/uint/struct.UInt.html\" title=\"struct typenum::uint::UInt\">UInt</a>&lt;Ul, Bl&gt;, Ur&gt;: Trim,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()