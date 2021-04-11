/*####################################################################
 * Abstract Data Type interfaces
 *
 * These are mainly interfaces defining how to use the built-in
 * JavaScript types to emulate more complex types.  For example, the
 * Dictionary interface just describes an object in which name/value
 * pairs are stored as properties.  Similarly, the ArraySet interface
 * just describes an array in which every element is assumed to be
 * unique.
 */
var hd;
(function (hd) {
    var utility;
    (function (utility) {
        var point;
        (function (point) {
            function difference(a, b) {
                return { x: a.x - b.x, y: a.y - b.y };
            }
            point.difference = difference;
        })(point = utility.point || (utility.point = {}));
        ;
        var multiArray;
        (function (multiArray) {
            /*------------------------------------------------------------------
             */
            function copy(ts) {
                var t2s = [];
                for (var i = 0, l = ts.length; i < l; ++i) {
                    if (Array.isArray(ts[i])) {
                        t2s.push(copy(ts[i]));
                    }
                    else {
                        t2s.push(ts[i]);
                    }
                }
                return t2s;
            }
            multiArray.copy = copy;
            /*------------------------------------------------------------------
             */
            function forEach(ts, fn, thisArg) {
                if (thisArg === void 0) { thisArg = null; }
                for (var i = 0, l = ts.length; i < l; ++i) {
                    if (Array.isArray(ts[i])) {
                        forEach(ts[i], fn, thisArg);
                    }
                    else {
                        fn.call(thisArg, ts[i], i, ts);
                    }
                }
            }
            multiArray.forEach = forEach;
            /*------------------------------------------------------------------
             */
            function map(ts, fn, thisArg) {
                if (thisArg === void 0) { thisArg = null; }
                var results = [];
                for (var i = 0, l = ts.length; i < l; ++i) {
                    if (Array.isArray(ts[i])) {
                        results.push(map(ts[i], fn, thisArg));
                    }
                    else {
                        results.push(fn.call(thisArg, ts[i], i, ts));
                    }
                }
                return results;
            }
            multiArray.map = map;
            /*----------------------------------------------------------------
             */
            function some(ts, fn, thisArg) {
                if (thisArg === void 0) { thisArg = null; }
                for (var i = 0, l = ts.length; i < l; ++i) {
                    if (Array.isArray(ts[i])) {
                        if (some(ts[i], fn, thisArg)) {
                            return true;
                        }
                    }
                    else {
                        if (fn.call(thisArg, ts[i], i, ts)) {
                            return true;
                        }
                    }
                }
                return false;
            }
            multiArray.some = some;
            /*----------------------------------------------------------------
             */
            function toString(ts) {
                var result = [];
                for (var i = 0, l = ts.length; i < l; ++i) {
                    if (i > 0) {
                        result.push(',');
                    }
                    if (Array.isArray(ts[i])) {
                        result.push('[');
                        result.push(toString(ts[i]));
                        result.push(']');
                    }
                    else {
                        result.push(ts[i]);
                    }
                }
                return result.join('');
            }
            multiArray.toString = toString;
            /*----------------------------------------------------------------
             */
            function flatten(from, to) {
                if (to === void 0) { to = []; }
                for (var i = 0, l = from.length; i < l; ++i) {
                    if (Array.isArray(from[i])) {
                        flatten(from[i], to);
                    }
                    else {
                        to.push(from[i]);
                    }
                }
                return to;
            }
            multiArray.flatten = flatten;
        })(multiArray = utility.multiArray || (utility.multiArray = {}));
        /*==================================================================
         * An ArraySet is simply an array where each element only occurs
         * in the array once.
         *
         * To make this, I copied from the standard "lib.d.ts" definitions
         * file all the members of the "Array" interface which make sense on
         * a set.  Thus, any array will automatically fuffill this
         * interface.
         *
         * Extra set functionality is provided through external functions.
         *
         * All ArraySet functions are in their own namespace to avoid
         * confusion.
         */
        var arraySet;
        (function (arraySet) {
            /*----------------------------------------------------------------
             * Make a copy.
             */
            function clone(as) {
                return as.slice(0);
            }
            arraySet.clone = clone;
            /*----------------------------------------------------------------
             * Test whether set contains element.
             */
            function contains(as, value) {
                return as.indexOf(value) != -1;
            }
            arraySet.contains = contains;
            /*----------------------------------------------------------------
             * Add element to set.
             * @returns the set
             * Useful with Array.reduce -- e.g.
             *   set = [3, 1, 4, 1, 5, 9, 3].reduce( arraySet.build, [] )
             */
            function build(as, value) {
                if (as.indexOf(value) == -1) {
                    as.push(value);
                }
                return as;
            }
            arraySet.build = build;
            /*----------------------------------------------------------------
             * Add element to set.
             * @returns true if element was added, false if it was already
             * contained by the set
             */
            function add(as, value) {
                if (as.indexOf(value) == -1) {
                    as.push(value);
                    return true;
                }
                else {
                    return false;
                }
            }
            arraySet.add = add;
            function addKnownDistinct(as, value) {
                as.push(value);
            }
            arraySet.addKnownDistinct = addKnownDistinct;
            /*----------------------------------------------------------------
             * Remove element from set.
             * @returns true if element was removed, false if it was not
             * contained by the set.
             */
            function remove(as, value) {
                var index = as.indexOf(value);
                if (index != -1) {
                    as.splice(index, 1);
                    return true;
                }
                else {
                    return false;
                }
            }
            arraySet.remove = remove;
            /*----------------------------------------------------------------
             * Calculate the union of two sets.
             */
            function union(as, bs) {
                return as.concat(bs.filter(function (b) {
                    return as.indexOf(b) == -1;
                }));
            }
            arraySet.union = union;
            /*----------------------------------------------------------------
             * Optimization for when we know the sets are disjoint to begin
             * with.
             */
            function unionKnownDisjoint(as, bs) {
                return as.concat(bs);
            }
            arraySet.unionKnownDisjoint = unionKnownDisjoint;
            /*----------------------------------------------------------------
             * Calculate the intersection of two sets.
             */
            function intersect(as, bs) {
                return as.filter(function (a) {
                    return bs.indexOf(a) != -1;
                });
            }
            arraySet.intersect = intersect;
            /*----------------------------------------------------------------
             * Calculate set difference.
             */
            function difference(as, bs) {
                return as.filter(function (a) {
                    return bs.indexOf(a) == -1;
                });
            }
            arraySet.difference = difference;
            /*----------------------------------------------------------------
             * Test for subset relation.
             */
            function isSubset(as, bs) {
                return as.every(function (a) {
                    return bs.indexOf(a) != -1;
                });
            }
            arraySet.isSubset = isSubset;
            /*----------------------------------------------------------------
             * Test for disjoint arrays.
             */
            function areDisjoint(as, bs) {
                return as.every(function (a) {
                    return bs.indexOf(a) == -1;
                });
            }
            arraySet.areDisjoint = areDisjoint;
            /*----------------------------------------------------------------
             * Test for equal arrays.
             */
            function areEqual(as, bs) {
                return as.length == bs.length && isSubset(as, bs);
            }
            arraySet.areEqual = areEqual;
            /*----------------------------------------------------------------
             * Build set from array (remove duplicates).
             */
            function fromArray(as) {
                // For some reason, TypeScript does not infer this type correctly
                var b = build;
                var f = as.reduce(b, []);
                return f;
            }
            arraySet.fromArray = fromArray;
        })(arraySet = utility.arraySet || (utility.arraySet = {}));
        /*==================================================================
         * A StringSet is simply an object used to store strings as property
         * names.  Since the value of the property does not matter, only the
         * property name, the boolean value "true" is used as the value;
         * this has the added bonus of simplifying the membership test.
         *
         * Extra set functionality is provided through external functions.
         *
         * All StringSet functions are in their own namespace to avoid
         * confusion.
         */
        var stringSet;
        (function (stringSet) {
            /*----------------------------------------------------------------
             * Make a copy.
             */
            function clone(ss) {
                var copy = {};
                for (var s in ss) {
                    if (ss[s]) {
                        copy[s] = true;
                    }
                }
                return copy;
            }
            stringSet.clone = clone;
            /*----------------------------------------------------------------
             * Test whether set contains element.
             */
            function contains(ss, s) {
                return ss[s];
            }
            stringSet.contains = contains;
            /*----------------------------------------------------------------
             * Add element to set.
             * @returns the set
             * Useful with Array.reduce -- e.g.
             *   set = ['hey', 'there', 'hi', 'there'].reduce( stringSet.build, [] )
             */
            function build(ss, s) {
                ss[s] = true;
                return ss;
            }
            stringSet.build = build;
            /*----------------------------------------------------------------
             * Add element to set.
             * @returns true if element was added, false if it was already
             * contained by the set
             */
            function add(ss, s) {
                if (ss[s]) {
                    return false;
                }
                else {
                    return ss[s] = true;
                }
            }
            stringSet.add = add;
            /*----------------------------------------------------------------
             * Remove element from set.
             * @returns true if element was removed, false if it was not
             * contained by the set.
             */
            function remove(ss, s) {
                if (ss[s]) {
                    delete ss[s];
                    return true;
                }
                else {
                    return false;
                }
            }
            stringSet.remove = remove;
            /*----------------------------------------------------------------
             * Gets all members of the set.
             */
            stringSet.members = Object.keys;
            /*----------------------------------------------------------------
             * Iteration
             */
            function forEach(ss, callback, thisArg) {
                if (thisArg === void 0) { thisArg = null; }
                for (var el in ss) {
                    callback.call(thisArg, el);
                }
            }
            stringSet.forEach = forEach;
            /*----------------------------------------------------------------
             * Calculate the union of two sets.
             */
            function union(as, bs) {
                var cs = {};
                for (var el in as) {
                    cs[el] = true;
                }
                for (var el in bs) {
                    cs[el] = true;
                }
                return cs;
            }
            stringSet.union = union;
            /*----------------------------------------------------------------
             * Calculate the intersection of two sets.
             */
            function intersect(as, bs) {
                var cs = {};
                for (var el in as) {
                    if (bs[el]) {
                        cs[el] = true;
                    }
                }
                return cs;
            }
            stringSet.intersect = intersect;
            /*----------------------------------------------------------------
             * Calculate set difference.
             */
            function difference(as, bs) {
                var cs = {};
                for (var el in as) {
                    if (!(el in bs)) {
                        cs[el] = true;
                    }
                }
                return cs;
            }
            stringSet.difference = difference;
            /*----------------------------------------------------------------
             * Build set from list of strings.
             */
            function fromArray(as) {
                return as.reduce(build, {});
            }
            stringSet.fromArray = fromArray;
        })(stringSet = utility.stringSet || (utility.stringSet = {}));
        /*==================================================================
         * This is a very simple queue implementation, basically storing
         * everything as an array with a varying initial index.
         */
        var Queue = /** @class */ (function () {
            function Queue() {
                this.begin = 0;
                this.end = 0;
            }
            /*----------------------------------------------------------------
             */
            Queue.prototype.isEmpty = function () {
                return this.begin == this.end;
            };
            /*----------------------------------------------------------------
             */
            Queue.prototype.isNotEmpty = function () {
                return this.begin != this.end;
            };
            /*----------------------------------------------------------------
             */
            Queue.prototype.enqueue = function (t) {
                this[this.end++] = t;
            };
            /*----------------------------------------------------------------
             */
            Queue.prototype.dequeue = function () {
                var t = this[this.begin];
                this[this.begin] = undefined;
                ++this.begin;
                if (this.begin >= this.end) {
                    this.begin = this.end = 0;
                }
                return t;
            };
            /*----------------------------------------------------------------
             */
            Queue.prototype.remove = function (t) {
                var removed = false;
                for (var i = this.begin, l = this.end; i < l; ++i) {
                    if (!removed && this[i] === t) {
                        removed = true;
                    }
                    if (removed) {
                        this[i] = this[i + 1];
                    }
                }
                if (removed) {
                    --this.end;
                    if (this.begin >= this.end) {
                        this.begin = this.end = 0;
                    }
                }
            };
            return Queue;
        }());
        utility.Queue = Queue;
        /*==================================================================
         */
        var Heap = /** @class */ (function () {
            function Heap(comesBefore) {
                this.members = [];
                this.length = 0;
                this.comesBefore = comesBefore;
            }
            Heap.prototype.contains = function (el) {
                return this.members.indexOf(el) >= 0;
            };
            Heap.prototype.push = function (el) {
                this.members.push(el);
                this.upheap(this.length);
                ++this.length;
            };
            Heap.prototype.pushAll = function (els) {
                for (var i = 0, l = els.length; i < l; ++i) {
                    this.members.push(els[i]);
                    this.upheap(this.length);
                    ++this.length;
                }
            };
            Heap.prototype.pop = function () {
                var result = this.members[0];
                var last = this.members.pop();
                if (--this.length > 0) {
                    this.members[0] = last;
                    this.downheap(0);
                }
                return result;
            };
            Heap.prototype.upheap = function (i) {
                var el = this.members[i];
                while (i > 0) {
                    var parentI = Math.floor((i + 1) / 2) - 1;
                    var parent = this.members[parentI];
                    if (this.comesBefore(el, parent)) {
                        this.members[parentI] = el;
                        this.members[i] = parent;
                        i = parentI;
                    }
                    else {
                        break;
                    }
                }
            };
            Heap.prototype.downheap = function (i) {
                var length = this.members.length;
                var el = this.members[i];
                while (true) {
                    var childI = 2 * i + 1;
                    if (childI < length) {
                        var child = this.members[childI];
                        var child2I = childI + 1;
                        if (child2I < length) {
                            var child2 = this.members[child2I];
                            if (this.comesBefore(child2, child)) {
                                childI = child2I;
                                child = child2;
                            }
                        }
                        if (this.comesBefore(child, el)) {
                            this.members[childI] = el;
                            this.members[i] = child;
                            i = childI;
                        }
                        else {
                            break;
                        }
                    }
                    else {
                        break;
                    }
                }
            };
            return Heap;
        }());
        utility.Heap = Heap;
    })(utility = hd.utility || (hd.utility = {}));
})(hd || (hd = {}));
/*####################################################################
 * Stand-alone helper functions, as well as a few generic interfaces.
 */
var hd;
(function (hd) {
    var utility;
    (function (utility) {
        /*------------------------------------------------------------------
         * There doesn't seem to be a built-in type for this.
         */
        /*------------------------------------------------------------------
         * Makes a "construction binder" - a function which takes its
         * arguments and binds them to a constructor, making a specialized
         * constructor.
         *
         * So this:
         *   fBinder = makeConstructionBinder( F );
         *   Fxy = fBinder( x, y );
         *   Fab = fBinder( a, b );
         *   f1 = new Fxy();
         *   f2 = new Fab();
         * is effectively the same as this:
         *   f1 = new F( x, y );
         *   f2 = new F( a, b );
         */
        function makeConstructorBinder(klass) {
            var args = [];
            for (var _i = 1; _i < arguments.length; _i++) {
                args[_i - 1] = arguments[_i];
            }
            var arg1 = Array.prototype.slice.call(arguments, 1);
            return function () {
                var arg2 = Array.prototype.slice.call(arguments, 0);
                return klass.bind.apply(klass, [null].concat(arg1, arg2));
            };
        }
        utility.makeConstructorBinder = makeConstructorBinder;
        var Fuzzy;
        (function (Fuzzy) {
            Fuzzy[Fuzzy["No"] = 0] = "No";
            Fuzzy[Fuzzy["Yes"] = 1] = "Yes";
            Fuzzy[Fuzzy["Maybe"] = 2] = "Maybe";
        })(Fuzzy = utility.Fuzzy || (utility.Fuzzy = {}));
        ;
        /*------------------------------------------------------------------
         * Does nothing.  Reuse this to avoid creating a bunch of empty
         * functions.
         */
        function noop() { }
        utility.noop = noop;
        /*------------------------------------------------------------------
         * Creates a duplicate of an object.  Note that this is a shallow
         * copy:  it's an object with the same prototype whose properties
         * point to the same values.
         */
        function shallowCopy(obj) {
            var orig = obj;
            var copy = Object.create(Object.getPrototypeOf(obj));
            for (var key in orig) {
                if (orig.hasOwnProperty(key)) {
                    copy[key] = (orig)[key];
                }
            }
            return copy;
        }
        utility.shallowCopy = shallowCopy;
        /*------------------------------------------------------------------
         * Creates a duplicate of an object.  Note that this is a deep
         * copy:  it will recursively copy any objects pointed to by this
         * object.  (So make sure there are no cycles.)
         */
        function deepCopy(obj) {
            var orig = obj;
            var copy = Array.isArray(obj)
                ? [] : Object.create(Object.getPrototypeOf(obj));
            for (var key in orig) {
                if (orig.hasOwnProperty(key)) {
                    if (typeof orig[key] === 'object') {
                        copy[key] = deepCopy(orig[key]);
                    }
                    else {
                        copy[key] = orig[key];
                    }
                }
            }
            return copy;
        }
        utility.deepCopy = deepCopy;
        /*------------------------------------------------------------------
         */
        function partition(ts, fn) {
            return ts.reduce(function build(d, t) {
                var key = fn(t);
                if (key in d) {
                    d[key].push(t);
                }
                else {
                    d[key] = [t];
                }
                return d;
            }, {});
        }
        utility.partition = partition;
        /*------------------------------------------------------------------
         * Map function over array and concatenate the results
         */
        function concatmap(ts, fn, thisArg) {
            if (thisArg === void 0) { thisArg = null; }
            var results = [];
            for (var i = 0, l = ts.length; i < l; ++i) {
                var result = fn.call(thisArg, ts[i], i, ts);
                Array.prototype.push.apply(results, Array.isArray(result) ? result : [result]);
            }
            return results;
        }
        utility.concatmap = concatmap;
        /*------------------------------------------------------------------
         * Map function over array in reverse order
         */
        function reversemap(ts, fn, thisArg) {
            if (thisArg === void 0) { thisArg = null; }
            var results = [];
            for (var i = ts.length - 1; i >= 0; --i) {
                results.push(fn.call(thisArg, ts[i], i, ts));
            }
            return results;
        }
        utility.reversemap = reversemap;
        function interpolate() {
            var as = [];
            for (var i = 0, l = arguments.length; i < l; ++i) {
                var a = arguments[i];
                if (Array.isArray(a)) {
                    as.push.apply(as, a);
                }
                else if (a !== undefined) {
                    as.push(a);
                }
            }
            return as;
        }
        utility.interpolate = interpolate;
        /*------------------------------------------------------------------
         * The compare function for numbers (most commonly used for sorting)
         */
        function numCompare(a, b) {
            return a - b;
        }
        utility.numCompare = numCompare;
        function dateCompare(a, b) {
            return a.valueOf() - b.valueOf();
        }
        utility.dateCompare = dateCompare;
        /*==================================================================
          The following functions are all simple functions intended to be
          used with the built-in array iteration functions
          (e.g. filter, map, etc.)
         *==================================================================*/
        /*------------------------------------------------------------------
         * Simple predicates to test type of object.  Written in curried
         * form.
         */
        function isType(type) {
            return function (obj) {
                return obj instanceof type;
            };
        }
        utility.isType = isType;
        function isNotType(type) {
            return function (obj) {
                return !(obj instanceof type);
            };
        }
        utility.isNotType = isNotType;
        /*------------------------------------------------------------------
         * Simple predicates to test for key in object.  Written in curried
         * form.
         */
        function nameIsIn(obj) {
            return function (name) {
                return name in obj;
            };
        }
        utility.nameIsIn = nameIsIn;
        function nameIsNotIn(obj) {
            return function (name) {
                return !(name in obj);
            };
        }
        utility.nameIsNotIn = nameIsNotIn;
        /*----------------------------------------------------------------
         * Simple function to get property with given name.  Written in
         * curried form.
         */
        function toValueIn(obj) {
            return function (name) {
                return obj[name];
            };
        }
        utility.toValueIn = toValueIn;
        /*------------------------------------------------------------------
         * Simple function to get the id property of an object.
         */
        function getId(obj) {
            return obj.id;
        }
        utility.getId = getId;
        /*------------------------------------------------------------------
         */
        function doubleEqual(a, b) {
            return a == b;
        }
        utility.doubleEqual = doubleEqual;
        /*------------------------------------------------------------------
         */
        function isNotNull(x) {
            return x !== null;
        }
        utility.isNotNull = isNotNull;
        function isUndefined(x) {
            return x === undefined;
        }
        utility.isUndefined = isUndefined;
    })(utility = hd.utility || (hd.utility = {}));
})(hd || (hd = {}));
/*####################################################################
 * Defines hd.utility.console for logging.
 *
 * This is a duplicate of window.console, with the addition that there
 * are certain content-specific consoles which can be either enabled
 * or disabled.  If they are disabled, then the logging functions are
 * still there, but they don't do anything.
 *
 * For example:
 *   // Always prints to console
 *   hd.utility.console.log( 'Hello' );
 *
 *   // Only prints if the "async" logger is enabled; otherwise no-op
 *   hd.utility.console.async.log( 'Hello' );
 *
 *   // Prints if at least one of "async", "solve", or "promise" is enabled
 *   hd.utility.console.async.solve.promise.log( 'Hello' );
 */
var hd;
(function (hd) {
})(hd || (hd = {}));
hd.globalConsole = console;
(function (hd) {
    var utility;
    (function (utility) {
        /*==================================================================
         * This class has two types of members.  First it has all the
         * logging functions from window.console.  Second it has links
         * to various content-specific consoles.
         */
        var Console = /** @class */ (function () {
            function Console() {
                // logging functions
                this.dir = utility.noop;
                this.error = utility.noop;
                this.group = utility.noop;
                this.groupCollapsed = utility.noop;
                this.groupEnd = utility.noop;
                this.info = utility.noop;
                this.log = utility.noop;
                this.time = utility.noop;
                this.timeEnd = utility.noop;
                this.trace = utility.noop;
                this.warning = utility.noop;
                // content-specific consoles
                this.async = this;
                this.compile = this;
            }
            return Console;
        }());
        utility.Console = Console;
        /*==================================================================
         * There are three consoles: the root console, the enabled console,
         * and the disabled console.
         */
        /*
         * The root console:
         * - Console functions come from window.console
         * - Enabled links point to enabled console
         * - Disabled links point to disabled console
         */
        utility.console = new Console();
        /*
         * The enabled console:
         * - Console functions come from window.console
         * - Enabled links point to enabled console
         * - Dsiabled links point to enabled console
         */
        var enabled = new Console();
        /*
         * The disabled console:
         * - Console functions are noop
         * - Enabled links point to enabled console
         * - Disabled links point to disabled console
         */
        var disabled = new Console();
        // This helper function creates a bound version of a specific
        // window.console function
        function bindConsoleFunction(name) {
            var consoleFns = hd.globalConsole;
            if (consoleFns[name]) {
                return consoleFns[name].bind(hd.globalConsole);
            }
        }
        // Set console functions for root/enabled consoles
        utility.console.dir = enabled.dir =
            bindConsoleFunction('dir');
        utility.console.error = enabled.error =
            bindConsoleFunction('error');
        utility.console.group = enabled.group =
            bindConsoleFunction('group');
        utility.console.groupCollapsed = enabled.groupCollapsed =
            bindConsoleFunction('groupCollapsed');
        utility.console.groupEnd = enabled.groupEnd =
            bindConsoleFunction('groupEnd');
        utility.console.info = enabled.info =
            bindConsoleFunction('info');
        utility.console.log = enabled.log =
            bindConsoleFunction('log');
        utility.console.time = enabled.time =
            bindConsoleFunction('time');
        utility.console.timeEnd = enabled.timeEnd =
            bindConsoleFunction('timeEnd');
        utility.console.trace = enabled.trace =
            bindConsoleFunction('trace');
        utility.console.warning = enabled.warning =
            bindConsoleFunction('warning');
        // Set console links for root/disabled
        enableConsole('async');
        enableConsole('info');
        enableConsole('log');
        enableConsole('time');
        enableConsole('timeEnd');
        enableConsole('trace');
        enableConsole('warning');
        disableConsole('compile');
        /*------------------------------------------------------------------
         * Enable a content-specific console
         */
        function enableConsole(name) {
            utility.console[name] =
                enabled[name];
        }
        utility.enableConsole = enableConsole;
        /*------------------------------------------------------------------
         * Disable a content-specific console
         */
        function disableConsole(name) {
            utility.console[name] =
                disabled[name] = disabled;
        }
        utility.disableConsole = disableConsole;
    })(utility = hd.utility || (hd.utility = {}));
})(hd || (hd = {}));
/*####################################################################
 * Exports the "schedule" function, which schedules a function to be
 * executed at the next available time after the current line of
 * execution has finished -- effectively window.setTimeout( fn, 0 )
 * or the non-standard window.setImmediate( fn ).
 *
 * Two ways this is different from setTimeout( fn, 0 ):
 * 1. Guarantees that, if schedule( f ) is called before schedule( g )
 *    then f will be called before g.
 * 2. Supports associating a priority with a function (0 = highest
 *    priority; increasing number = decreasing priority); will always
 *    run higher priority function first regardless of when it was
 *    scheduled.
 */
var hd;
(function (hd) {
    var utility;
    (function (utility) {
        /*==================================================================
         * Remembers everything needed to perform a single requested
         * execution.
         */
        var ScheduledTask = /** @class */ (function () {
            function ScheduledTask(priority, fn, thisArg, params) {
                this.priority = priority;
                this.fn = fn;
                this.thisArg = thisArg;
                this.params = params;
            }
            ScheduledTask.prototype.run = function () {
                try {
                    this.fn.apply(this.thisArg, this.params);
                }
                catch (e) {
                    utility.console.error(e);
                }
            };
            return ScheduledTask;
        }());
        utility.ScheduledTask = ScheduledTask;
        /*==================================================================
         * Invoke the next scheduled task
         */
        // queues, indexed by their priority
        var taskqueues = [];
        // total number of tasks queued up
        var taskcount = 0;
        // id of the current timeout
        var timerId = null;
        function runTasks() {
            do {
                var task = null;
                // get the next task from the first non-empty queue
                for (var i = 0; task === null && i < taskqueues.length; ++i) {
                    var queue = taskqueues[i];
                    if (queue && queue.isNotEmpty()) {
                        task = queue.dequeue();
                    }
                }
                // run the task
                if (task) {
                    task.run();
                    --taskcount;
                }
                else {
                    taskcount = 0;
                }
            } while (taskcount > 0);
            if (taskcount) {
                timerId = setTimeout(runTasks, 0);
            }
            else {
                timerId = null;
            }
        }
        /*==================================================================
         * Schedule a function to be run later.
         */
        function schedule(priority, fn, thisArg) {
            var params = [];
            for (var _i = 3; _i < arguments.length; _i++) {
                params[_i - 3] = arguments[_i];
            }
            if (priority < 0) {
                try {
                    fn.apply(thisArg, params);
                }
                catch (e) {
                    utility.console.error(e);
                }
                return null;
            }
            else {
                var task = new ScheduledTask(priority, fn, thisArg, params);
                // add the task to the approriate queue
                var queue = taskqueues[priority];
                if (queue === undefined) {
                    queue = taskqueues[priority] = new utility.Queue();
                }
                queue.enqueue(task);
                ++taskcount;
                // set timer to run all scheduled tasks
                if (!timerId) {
                    timerId = setTimeout(runTasks, 0);
                }
                return task;
            }
        }
        utility.schedule = schedule;
        function deschedule(task) {
            var queue = taskqueues[task.priority];
            if (queue) {
                queue.remove(task);
            }
        }
        utility.deschedule = deschedule;
    })(utility = hd.utility || (hd.utility = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var u = hd.utility;
    hd.dateCompare = u.dateCompare;
})(hd || (hd = {}));
//
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (b.hasOwnProperty(p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
/*####################################################################
 * Interfaces related to Observable design pattern.
 */
var hd;
(function (hd) {
    var reactive;
    (function (reactive) {
        reactive.ObservablePriority = -1;
        /*==================================================================
         */
        var ProxyObserver = /** @class */ (function () {
            function ProxyObserver(object, next_cb, error_cb, completed_cb, id) {
                this.object = object;
                this.next_cb = next_cb;
                this.error_cb = error_cb;
                this.completed_cb = completed_cb;
                this.id = id;
            }
            ProxyObserver.prototype.onNext = function (value) {
                if (this.next_cb) {
                    this.next_cb.call(this.object, value, this.id);
                }
            };
            ProxyObserver.prototype.onError = function (error) {
                if (this.error_cb) {
                    this.error_cb.call(this.object, error, this.id);
                }
            };
            ProxyObserver.prototype.onCompleted = function () {
                if (this.completed_cb) {
                    this.completed_cb.call(this.object, this.id);
                }
            };
            return ProxyObserver;
        }());
        reactive.ProxyObserver = ProxyObserver;
        /*==================================================================
         * Straightforward implementation of Observable.
         */
        var BasicObservable = /** @class */ (function () {
            function BasicObservable() {
            }
            /*----------------------------------------------------------------
             * Predicate to tell if anyone's listening
             */
            BasicObservable.prototype.hasObservers = function () {
                return this.observers.length > 0;
            };
            BasicObservable.prototype.addObserver = function (object, onNext, onError, onCompleted, id) {
                var observer;
                if (arguments.length == 1) {
                    observer = object;
                }
                else {
                    observer = new ProxyObserver(object, onNext, onError, onCompleted, id);
                }
                if (this.hasOwnProperty('observers')) {
                    this.observers.push(observer);
                }
                else {
                    // copy on write
                    this.observers = [observer];
                }
                return observer;
            };
            /*----------------------------------------------------------------
             * Cancel subscription by a particular observer.
             */
            BasicObservable.prototype.removeObserver = function (observer) {
                var removed = false;
                for (var i = this.observers.length; i >= 0; --i) {
                    var o = this.observers[i];
                    if (o === observer ||
                        (o instanceof ProxyObserver &&
                            o.object === observer)) {
                        this.observers.splice(i, 1);
                        removed = true;
                    }
                }
                return removed;
            };
            /*----------------------------------------------------------------
             * Send "next" event to all observers.
             */
            BasicObservable.prototype.sendNext = function (value) {
                this.observers.slice(0).forEach(function (observer) {
                    if (observer.onNext) {
                        observer.onNext(value);
                        //u.schedule( ObservablePriority, observer.onNext, observer, value );
                    }
                });
            };
            /*----------------------------------------------------------------
             * Send "error" event to all observers.
             */
            BasicObservable.prototype.sendError = function (error) {
                this.observers.slice(0).forEach(function (observer) {
                    if (observer.onError) {
                        observer.onError(error);
                        //u.schedule( ObservablePriority, observer.onError, observer, error );
                    }
                });
            };
            /*----------------------------------------------------------------
             * Send "completed" event to all observers.
             */
            BasicObservable.prototype.sendCompleted = function () {
                this.observers.slice(0).forEach(function (observer) {
                    if (observer.onCompleted) {
                        observer.onCompleted();
                        //u.schedule( ObservablePriority, observer.onCompleted, observer );
                    }
                });
            };
            return BasicObservable;
        }());
        reactive.BasicObservable = BasicObservable;
        // Inherited empty observer list
        BasicObservable.prototype.observers = [];
        /*==================================================================
         * Combine list of ovservables into a single observable
         */
        var Union = /** @class */ (function (_super) {
            __extends(Union, _super);
            /*----------------------------------------------------------------
             * Watch everything
             */
            function Union(sources) {
                var _this = _super.call(this) || this;
                // Number of observables completed
                _this.completedCount = 0;
                _this.count = sources.length;
                sources.forEach(function (obs) {
                    obs.addObserver(this);
                });
                return _this;
            }
            /*----------------------------------------------------------------
             * Pass along.
             */
            Union.prototype.onNext = function (value) {
                this.sendNext(value);
            };
            /*----------------------------------------------------------------
             * Pass along.
             */
            Union.prototype.onError = function (error) {
                this.sendError(error);
            };
            /*----------------------------------------------------------------
             * Wait for the last; then pass along.
             */
            Union.prototype.onCompleted = function () {
                if (++this.completedCount == this.count) {
                    this.sendCompleted();
                }
            };
            return Union;
        }(BasicObservable));
        reactive.Union = Union;
    })(reactive = hd.reactive || (hd.reactive = {}));
})(hd || (hd = {}));
/*####################################################################
 * The Signal class.
 */
var hd;
(function (hd) {
    var reactive;
    (function (reactive) {
        var u = hd.utility;
        reactive.SignalPriority = 3;
        var Scheduled;
        (function (Scheduled) {
            Scheduled[Scheduled["None"] = 0] = "None";
            Scheduled[Scheduled["Init"] = 1] = "Init";
            Scheduled[Scheduled["Update"] = 2] = "Update";
        })(Scheduled || (Scheduled = {}));
        ;
        var BasicSignal = /** @class */ (function (_super) {
            __extends(BasicSignal, _super);
            function BasicSignal(value, eq) {
                var _this = _super.call(this) || this;
                _this.value = value;
                if (eq) {
                    _this.eq = eq;
                }
                return _this;
            }
            BasicSignal.prototype.addObserver = function (object, onNext, onError, onCompleted, id) {
                var added;
                if (arguments.length === 1) {
                    added = _super.prototype.addObserver.call(this, object);
                }
                else {
                    added = _super.prototype.addObserver.call(this, object, onNext, onError, onCompleted, id);
                }
                if (added && this.value !== undefined) {
                    added.onNext(this.value);
                }
                return added;
            };
            BasicSignal.prototype.addObserverChangesOnly = function (object, onNext, onError, onCompleted, id) {
                var added;
                if (arguments.length === 1) {
                    added = _super.prototype.addObserver.call(this, object);
                }
                else {
                    added = _super.prototype.addObserver.call(this, object, onNext, onError, onCompleted, id);
                }
                return added;
            };
            /*----------------------------------------------------------------
             * Setter.  Sends a "next" notification if the value has changed.
             */
            BasicSignal.prototype.set = function (value) {
                if (!this.hasValue(value)) {
                    this.value = value;
                    this.sendNext(value);
                }
            };
            /*----------------------------------------------------------------
             * Getter.
             */
            BasicSignal.prototype.get = function () {
                return this.value;
            };
            /*----------------------------------------------------------------
             * Comparison.
             */
            BasicSignal.prototype.hasValue = function (value) {
                if (this.eq) {
                    return this.eq(this.value, value);
                }
                else {
                    return this.value === undefined && value === undefined;
                }
            };
            return BasicSignal;
        }(reactive.BasicObservable));
        reactive.BasicSignal = BasicSignal;
        /*==================================================================
         * An observable value that belongs to an object.
         */
        var ScheduledSignal = /** @class */ (function (_super) {
            __extends(ScheduledSignal, _super);
            /*----------------------------------------------------------------
             * Initialize properties
             */
            function ScheduledSignal(value, eq) {
                var _this = _super.call(this) || this;
                // Is there an update currently scheduled?
                _this.scheduled = Scheduled.None;
                _this.needInit = null;
                _this.value = value;
                if (eq) {
                    _this.eq = eq;
                }
                return _this;
            }
            /*----------------------------------------------------------------
             * Used to schedule an "onNext" notification for either an
             * individual observer or else all observers.
             */
            ScheduledSignal.prototype.scheduleUpdate = function () {
                if (this.scheduled === Scheduled.None) {
                    this.scheduled = Scheduled.Update;
                    u.schedule(reactive.SignalPriority, this.update, this);
                }
                else {
                    this.scheduled = Scheduled.Update;
                }
            };
            ScheduledSignal.prototype.scheduleInit = function (observer) {
                if (this.needInit) {
                    this.needInit.push(observer);
                }
                else {
                    this.needInit = [observer];
                }
                if (this.scheduled === Scheduled.None) {
                    this.scheduled = Scheduled.Init;
                    u.schedule(reactive.SignalPriority, this.update, this);
                }
            };
            /*----------------------------------------------------------------
             * Sends "onNext" notification to some or all observers.
             */
            ScheduledSignal.prototype.update = function () {
                // In case callbacks should modify this signal, we store these locally and reset them
                var scheduled = this.scheduled;
                this.scheduled = Scheduled.None;
                var init = this.needInit;
                this.needInit = null;
                if (scheduled === Scheduled.Update) {
                    if (this.hasValue(this.lastUpdate)) {
                        scheduled = Scheduled.Init;
                    }
                    else {
                        this.lastUpdate = this.value;
                        this.sendNext(this.value);
                    }
                }
                if (scheduled === Scheduled.Init && init) {
                    init.forEach(function (observer) {
                        u.schedule(reactive.ObservablePriority, observer.onNext, observer, this.value);
                    }, this);
                }
            };
            ScheduledSignal.prototype.addObserver = function (object, onNext, onError, onCompleted, id) {
                var added;
                if (arguments.length === 1) {
                    added = _super.prototype.addObserver.call(this, object);
                }
                else {
                    added = _super.prototype.addObserver.call(this, object, onNext, onError, onCompleted, id);
                }
                if (added && this.value !== undefined) {
                    this.scheduleInit(added);
                }
                return added;
            };
            ScheduledSignal.prototype.addObserverChangesOnly = function (object, onNext, onError, onCompleted, id) {
                var added;
                if (arguments.length === 1) {
                    added = _super.prototype.addObserver.call(this, object);
                }
                else {
                    added = _super.prototype.addObserver.call(this, object, onNext, onError, onCompleted, id);
                }
                return added;
            };
            /*----------------------------------------------------------------
             * Setter.  Sends a "next" notification if the value has changed.
             */
            ScheduledSignal.prototype.set = function (value) {
                if (!this.hasValue(value)) {
                    this.value = value;
                    this.scheduleUpdate();
                }
            };
            /*----------------------------------------------------------------
             * Getter.
             */
            ScheduledSignal.prototype.get = function () {
                return this.value;
            };
            /*----------------------------------------------------------------
             * Comparison.
             */
            ScheduledSignal.prototype.hasValue = function (value) {
                if (this.eq) {
                    return this.eq(this.value, value);
                }
                else {
                    return false;
                }
            };
            return ScheduledSignal;
        }(reactive.BasicObservable));
        reactive.ScheduledSignal = ScheduledSignal;
    })(reactive = hd.reactive || (hd.reactive = {}));
})(hd || (hd = {}));
/*####################################################################
 * Extensions are Observers and Observables.  They take events
 * produced by an Observable, modify them somehow, and then pass them
 * on.
 */
var hd;
(function (hd) {
    var reactive;
    (function (reactive) {
        /*==================================================================
         * Base class - simply take an event and pass it on.
         *
         * Derived classes can simply modify any events that are needed.
         */
        var Extension = /** @class */ (function (_super) {
            __extends(Extension, _super);
            function Extension() {
                return _super !== null && _super.apply(this, arguments) || this;
            }
            Extension.prototype.onNext = function (value) {
                this.sendNext(value);
            };
            Extension.prototype.onError = function (error) {
                this.sendError(error);
            };
            Extension.prototype.onCompleted = function () {
                this.sendCompleted();
            };
            return Extension;
        }(reactive.BasicObservable));
        reactive.Extension = Extension;
        /*------------------------------------------------------------------
         * Creates an extension from a single transformation function.
         */
        var FunctionExtension = /** @class */ (function (_super) {
            __extends(FunctionExtension, _super);
            function FunctionExtension(fn, thisArg, boundArgs) {
                var _this = _super.call(this) || this;
                _this.fn = (boundArgs && boundArgs.length) ? fn.bind.apply(fn, [thisArg || null].concat(boundArgs)) : fn;
                return _this;
            }
            FunctionExtension.prototype.onNext = function (value) {
                try {
                    this.sendNext(this.fn(value));
                }
                catch (e) {
                    this.sendError(e);
                }
            };
            FunctionExtension.prototype.onError = function (error) {
                this.sendError(error);
            };
            FunctionExtension.prototype.onCompleted = function () {
                this.sendCompleted();
            };
            return FunctionExtension;
        }(Extension));
        reactive.FunctionExtension = FunctionExtension;
        /*==================================================================
         */
        var Constant = /** @class */ (function () {
            function Constant(value) {
                this.value = value;
            }
            Constant.prototype.addObserver = function (observer) {
                observer.onNext(this.value);
            };
            Constant.prototype.removeObserver = function (observer) { };
            Constant.prototype.onNext = function () { };
            Constant.prototype.onError = function () { };
            Constant.prototype.onCompleted = function () { };
            return Constant;
        }());
        reactive.Constant = Constant;
        /*==================================================================
         * Basically a linked list of extensions, treated as a single
         * extension.  Can be empty, in which case it just acts as a
         * pass-through.
         */
        var Chain = /** @class */ (function (_super) {
            __extends(Chain, _super);
            /*----------------------------------------------------------------
             * Link together
             */
            function Chain(exts) {
                var _this = _super.call(this) || this;
                _this.exts = exts = exts ? exts.slice(0) : [];
                for (var i = 1, l = exts.length; i < l; ++i) {
                    exts[i - 1].addObserver(exts[i]);
                }
                if (i > 1) {
                    _this.forward(exts[l - 1]);
                }
                return _this;
            }
            /*----------------------------------------------------------------
             * Observe and pass on.
             */
            Chain.prototype.forward = function (ext) {
                ext.addObserver(this, this.sendNext, this.sendError, this.sendCompleted);
            };
            /*----------------------------------------------------------------
             * Pass values to head of the list.
             */
            Chain.prototype.onNext = function (value) {
                if (this.exts.length) {
                    this.exts[0].onNext(value);
                }
                else {
                    this.sendNext(value);
                }
            };
            /*----------------------------------------------------------------
             * Pass errors to head of the list.
             */
            Chain.prototype.onError = function (error) {
                if (this.exts.length) {
                    this.exts[0].onError(error);
                }
                else {
                    this.sendError(error);
                }
            };
            /*----------------------------------------------------------------
             * Pass completed to head of the list.
             */
            Chain.prototype.onCompleted = function () {
                if (this.exts.length) {
                    this.exts[0].onCompleted();
                }
                else {
                    this.sendCompleted();
                }
            };
            /*----------------------------------------------------------------
             * Add new extension to the tail of the list
             */
            Chain.prototype.push = function (ext) {
                if (this.exts.length) {
                    var last = this.exts[this.exts.length - 1];
                    last.removeObserver(this);
                    last.addObserver(ext);
                }
                this.forward(ext);
                this.exts.push(ext);
            };
            /*----------------------------------------------------------------
             * Remove extension from the tail of the list
             */
            Chain.prototype.pop = function () {
                if (this.exts.length) {
                    var ext = this.exts.pop();
                    ext.removeObserver(this);
                    if (this.exts.length) {
                        this.forward(this.exts[this.exts.length - 1]);
                    }
                }
                return ext;
            };
            /*----------------------------------------------------------------
             * Add new extension to the head of the list
             */
            Chain.prototype.unshift = function (ext) {
                if (this.exts.length) {
                    ext.addObserver(this.exts[0]);
                }
                else {
                    this.forward(ext);
                }
                this.exts.unshift(ext);
            };
            /*----------------------------------------------------------------
             * Remove extension from the tail of the list
             */
            Chain.prototype.shift = function () {
                if (this.exts.length) {
                    var ext = this.exts.shift();
                    if (this.exts.length) {
                        ext.removeObserver(this.exts[0]);
                    }
                    else {
                        ext.removeObserver(this);
                    }
                }
                return ext;
            };
            return Chain;
        }(reactive.BasicObservable));
        reactive.Chain = Chain;
        /*==================================================================
         */
        var HotSwapObservable = /** @class */ (function (_super) {
            __extends(HotSwapObservable, _super);
            function HotSwapObservable(source) {
                var _this = _super.call(this) || this;
                _this.proxy = new reactive.ProxyObserver(_this, _this.onSourceNext, _this.onSourceError, _this.onSourceCompleted);
                _this.source = source;
                if (source) {
                    source.addObserver(_this.proxy);
                }
                return _this;
            }
            HotSwapObservable.prototype.onSourceNext = function (value) {
                this.sendNext(value);
            };
            HotSwapObservable.prototype.onSourceError = function (error) {
                this.sendError(error);
            };
            HotSwapObservable.prototype.onSourceCompleted = function () {
                this.sendCompleted();
            };
            HotSwapObservable.prototype.onNext = function (source) {
                if (this.source) {
                    this.source.removeObserver(this.proxy);
                }
                this.source = source;
                if (source) {
                    source.addObserver(this.proxy);
                }
            };
            HotSwapObservable.prototype.onError = function () { };
            HotSwapObservable.prototype.onCompleted = function () { };
            return HotSwapObservable;
        }(reactive.BasicObservable));
        reactive.HotSwapObservable = HotSwapObservable;
        var HotSwapSignal = /** @class */ (function (_super) {
            __extends(HotSwapSignal, _super);
            function HotSwapSignal() {
                return _super !== null && _super.apply(this, arguments) || this;
            }
            HotSwapSignal.prototype.get = function () {
                return this.source ? this.source.get() : undefined;
            };
            HotSwapSignal.prototype.onNext = function (source) {
                var oldsource = this.source;
                this.source = source;
                if (oldsource) {
                    oldsource.removeObserver(this.proxy);
                }
                if (source) {
                    source.addObserver(this.proxy);
                }
                if (oldsource && !source) {
                    this.sendNext(undefined);
                }
            };
            return HotSwapSignal;
        }(HotSwapObservable));
        reactive.HotSwapSignal = HotSwapSignal;
        var HotSwap = /** @class */ (function (_super) {
            __extends(HotSwap, _super);
            function HotSwap(source) {
                var _this = _super.call(this) || this;
                _this.source = source;
                _this.target = null;
                source.addObserver(_this, _this.onNextTarget, null, null);
                if (source) {
                    _this.onNextTarget(source.get());
                }
                return _this;
            }
            HotSwap.prototype.addObserver = function () {
                var o = _super.prototype.addObserver.apply(this, arguments);
                if (this.last !== undefined) {
                    o.onNext(this.last);
                }
                return o;
            };
            HotSwap.prototype.onNextTarget = function (target) {
                if (this.target) {
                    this.target.removeObserver(this);
                }
                this.target = target;
                if (this.target) {
                    target.addObserver(this, this.onTargetNext, this.onTargetError, this.onTargetCompleted);
                }
            };
            HotSwap.prototype.onNext = function (value) {
                if (this.target) {
                    this.target.onNext(value);
                }
            };
            HotSwap.prototype.onError = function (error) {
                if (this.target) {
                    this.target.onError(error);
                }
            };
            HotSwap.prototype.onCompleted = function () {
                if (this.target) {
                    this.target.onCompleted();
                }
            };
            HotSwap.prototype.onTargetNext = function (value) {
                this.last = value;
                this.sendNext(value);
            };
            HotSwap.prototype.onTargetError = function (error) {
                this.sendError(error);
            };
            HotSwap.prototype.onTargetCompleted = function () {
                this.sendCompleted();
                this.target = null;
            };
            return HotSwap;
        }(reactive.BasicObservable));
        reactive.HotSwap = HotSwap;
        /*==================================================================
         */
        var ReadWrite = /** @class */ (function (_super) {
            __extends(ReadWrite, _super);
            function ReadWrite(read, write) {
                var _this = _super.call(this) || this;
                _this.read = read;
                _this.write = write;
                return _this;
            }
            ReadWrite.prototype.addObserver = function () {
                return this.read.addObserver.apply(this.read, arguments);
            };
            ReadWrite.prototype.removeObserver = function (observer) {
                this.read.removeObserver(observer);
                return true;
            };
            ReadWrite.prototype.onNext = function (value) {
                this.write.onNext(value);
            };
            ReadWrite.prototype.onError = function (error) {
                this.write.onNext(error);
            };
            ReadWrite.prototype.onCompleted = function () {
                this.write.onCompleted();
            };
            return ReadWrite;
        }(Extension));
        reactive.ReadWrite = ReadWrite;
        /*==================================================================
         */
        var Delay = /** @class */ (function (_super) {
            __extends(Delay, _super);
            function Delay(time_ms) {
                var _this = _super.call(this) || this;
                _this.time_ms = time_ms;
                return _this;
            }
            Delay.prototype.onNext = function (value) {
                setTimeout(this.sendNext.bind(this, value), this.time_ms);
            };
            Delay.prototype.onError = function (error) {
                setTimeout(this.sendError.bind(this, error), this.time_ms);
            };
            Delay.prototype.onCompleted = function () {
                setTimeout(this.sendCompleted.bind(this), this.time_ms);
            };
            return Delay;
        }(Extension));
        reactive.Delay = Delay;
        /*==================================================================
         * Cuts down the number of events produced.  When it receives an
         * event, it waits a specified amount of time.  If other events come
         * in during that time period then it discards the first event; if
         * not, then it passes it on.
         */
        var StabilizerState;
        (function (StabilizerState) {
            StabilizerState[StabilizerState["None"] = 0] = "None";
            StabilizerState[StabilizerState["Next"] = 1] = "Next";
            StabilizerState[StabilizerState["Error"] = 2] = "Error";
        })(StabilizerState || (StabilizerState = {}));
        ;
        var Stabilizer = /** @class */ (function (_super) {
            __extends(Stabilizer, _super);
            /*----------------------------------------------------------------
             * Initialize
             */
            function Stabilizer(time_ms) {
                if (time_ms === void 0) { time_ms = 400; }
                var _this = _super.call(this) || this;
                _this.time = time_ms;
                return _this;
            }
            /*----------------------------------------------------------------
             * Throw away any events currently being waited on, and wait on
             * this one.
             */
            Stabilizer.prototype.onNext = function (value) {
                if (!this.promise || this.promise.isSettled()) {
                    this.promise = new reactive.AccumulatingPromise();
                    this.promise.setDelay(this.time);
                    this.sendNext(this.promise);
                }
                this.promise.updateResolve(value);
            };
            /*----------------------------------------------------------------
             * Throw away any events currently being waited on, and wait on
             * this one.
             */
            Stabilizer.prototype.onError = function (error) {
                if (!this.promise || this.promise.isSettled()) {
                    this.promise = new reactive.AccumulatingPromise();
                    this.promise.setDelay(this.time);
                    this.sendNext(this.promise);
                }
                this.promise.updateReject(error);
            };
            /*----------------------------------------------------------------
             * Fire any events currently being waited on, and then the
             * "completed" event.
             */
            Stabilizer.prototype.onCompleted = function () {
                if (this.promise) {
                    this.promise.settle();
                    this.promise = null;
                }
            };
            return Stabilizer;
        }(Extension));
        reactive.Stabilizer = Stabilizer;
        /*==================================================================
         */
        var ReplaceError = /** @class */ (function (_super) {
            __extends(ReplaceError, _super);
            function ReplaceError(replacement) {
                var _this = _super.call(this) || this;
                _this.replacement = replacement;
                return _this;
            }
            ReplaceError.prototype.onError = function (value) {
                this.sendError(this.replacement);
            };
            return ReplaceError;
        }(Extension));
        reactive.ReplaceError = ReplaceError;
        var Required = /** @class */ (function (_super) {
            __extends(Required, _super);
            function Required() {
                return _super !== null && _super.apply(this, arguments) || this;
            }
            Required.prototype.onNext = function (value) {
                if (value === undefined || value === null || value === '') {
                    this.sendError('Required');
                }
                else {
                    this.sendNext(value);
                }
            };
            return Required;
        }(Extension));
        reactive.Required = Required;
        var Default = /** @class */ (function (_super) {
            __extends(Default, _super);
            function Default(defaultValue) {
                var _this = _super.call(this) || this;
                _this.defaultValue = defaultValue;
                return _this;
            }
            Default.prototype.onNext = function (value) {
                if (value === undefined || value === null || value === '') {
                    this.sendNext(this.defaultValue);
                }
                else {
                    this.sendNext(value);
                }
            };
            Default.prototype.onError = function () {
                this.sendNext(this.defaultValue);
            };
            return Default;
        }(Extension));
        reactive.Default = Default;
        /*==================================================================
         * Convert value to string using toString method.
         */
        var ToString = /** @class */ (function (_super) {
            __extends(ToString, _super);
            function ToString() {
                return _super !== null && _super.apply(this, arguments) || this;
            }
            ToString.prototype.onNext = function (value) {
                if (value === undefined || value === null) {
                    this.sendNext("");
                }
                else {
                    this.sendNext(value.toString());
                }
            };
            return ToString;
        }(Extension));
        reactive.ToString = ToString;
        /*==================================================================
         * Convert value to JSON string.
         */
        var ToJson = /** @class */ (function (_super) {
            __extends(ToJson, _super);
            function ToJson() {
                return _super !== null && _super.apply(this, arguments) || this;
            }
            ToJson.prototype.onNext = function (value) {
                this.sendNext(JSON.stringify(value));
            };
            return ToJson;
        }(Extension));
        reactive.ToJson = ToJson;
        /*==================================================================
         * Round number to specified number of digits after decimal point.
         */
        var Round = /** @class */ (function (_super) {
            __extends(Round, _super);
            function Round(places) {
                if (places === void 0) { places = 0; }
                var _this = _super.call(this) || this;
                _this.scale = 1;
                if (places < 0) {
                    for (var i = 0, l = places; i > l; --i) {
                        _this.scale /= 10;
                    }
                }
                else {
                    for (var i = 0, l = places; i < l; ++i) {
                        _this.scale *= 10;
                    }
                }
                return _this;
            }
            Round.prototype.onNext = function (value) {
                this.sendNext(Math.round(value * this.scale) / this.scale);
            };
            return Round;
        }(Extension));
        reactive.Round = Round;
        /*==================================================================
         * Convert number to string with fixed number of decimal places.
         */
        var NumberToFixed = /** @class */ (function (_super) {
            __extends(NumberToFixed, _super);
            function NumberToFixed(places) {
                var _this = _super.call(this) || this;
                _this.places = places;
                return _this;
            }
            NumberToFixed.prototype.onNext = function (value) {
                this.sendNext(value.toFixed(this.places));
            };
            return NumberToFixed;
        }(Extension));
        reactive.NumberToFixed = NumberToFixed;
        /*==================================================================
         * Convert number to string with given precision.
         */
        var NumberToPrecision = /** @class */ (function (_super) {
            __extends(NumberToPrecision, _super);
            function NumberToPrecision(sigfigs) {
                var _this = _super.call(this) || this;
                _this.sigfigs = sigfigs;
                return _this;
            }
            NumberToPrecision.prototype.onNext = function (value) {
                this.sendNext(value.toPrecision(this.sigfigs));
            };
            return NumberToPrecision;
        }(Extension));
        reactive.NumberToPrecision = NumberToPrecision;
        /*==================================================================
         * Convert number to fixed using exponential notation.
         */
        var NumberToExponential = /** @class */ (function (_super) {
            __extends(NumberToExponential, _super);
            function NumberToExponential(places) {
                var _this = _super.call(this) || this;
                _this.places = places;
                return _this;
            }
            NumberToExponential.prototype.onNext = function (value) {
                this.sendNext(value.toExponential(this.places));
            };
            return NumberToExponential;
        }(Extension));
        reactive.NumberToExponential = NumberToExponential;
        /*==================================================================
         * Convert string value to number.
         */
        var ToNumber = /** @class */ (function (_super) {
            __extends(ToNumber, _super);
            function ToNumber() {
                return _super !== null && _super.apply(this, arguments) || this;
            }
            ToNumber.prototype.onNext = function (value) {
                var n = Number(value);
                if (value == '' || isNaN(n) || n === Infinity) {
                    this.sendError("Invalid number");
                }
                else {
                    this.sendNext(n);
                }
            };
            return ToNumber;
        }(Extension));
        reactive.ToNumber = ToNumber;
        /*==================================================================
         * Scale a number by a constant factor.
         */
        var ScaleNumber = /** @class */ (function (_super) {
            __extends(ScaleNumber, _super);
            function ScaleNumber(scale) {
                var _this = _super.call(this) || this;
                _this.scale = 1;
                _this.scale = scale;
                return _this;
            }
            ScaleNumber.prototype.onNext = function (value) {
                this.sendNext(this.scale * value);
            };
            return ScaleNumber;
        }(Extension));
        reactive.ScaleNumber = ScaleNumber;
        /*==================================================================
         * Convert value to date.
         */
        var ToDate = /** @class */ (function (_super) {
            __extends(ToDate, _super);
            function ToDate() {
                return _super !== null && _super.apply(this, arguments) || this;
            }
            ToDate.prototype.onNext = function (value) {
                var d;
                if (value == '' || isNaN((d = new Date(value)).getTime())) {
                    this.sendError("Invalid date");
                }
                else {
                    this.sendNext(d);
                }
            };
            return ToDate;
        }(Extension));
        reactive.ToDate = ToDate;
        /*==================================================================
         * Convert date value to string.
         */
        var DateToString = /** @class */ (function (_super) {
            __extends(DateToString, _super);
            function DateToString() {
                return _super !== null && _super.apply(this, arguments) || this;
            }
            DateToString.prototype.onNext = function (value) {
                this.sendNext(value ? value.toLocaleString() : '');
            };
            return DateToString;
        }(Extension));
        reactive.DateToString = DateToString;
        /*==================================================================
         */
        var DateToDateString = /** @class */ (function (_super) {
            __extends(DateToDateString, _super);
            function DateToDateString() {
                return _super !== null && _super.apply(this, arguments) || this;
            }
            DateToDateString.prototype.onNext = function (value) {
                this.sendNext(value ? value.toLocaleDateString() : '');
            };
            return DateToDateString;
        }(Extension));
        reactive.DateToDateString = DateToDateString;
        /*==================================================================
         */
        var DateToTimeString = /** @class */ (function (_super) {
            __extends(DateToTimeString, _super);
            function DateToTimeString() {
                return _super !== null && _super.apply(this, arguments) || this;
            }
            DateToTimeString.prototype.onNext = function (value) {
                this.sendNext(value ? value.toLocaleTimeString() : '');
            };
            return DateToTimeString;
        }(Extension));
        reactive.DateToTimeString = DateToTimeString;
        /*==================================================================
         * Converts milliseconds to date string.
         */
        var DateToMilliseconds = /** @class */ (function (_super) {
            __extends(DateToMilliseconds, _super);
            function DateToMilliseconds() {
                return _super !== null && _super.apply(this, arguments) || this;
            }
            DateToMilliseconds.prototype.onNext = function (value) {
                this.sendNext(value ? value.getTime() : 0);
            };
            return DateToMilliseconds;
        }(Extension));
        reactive.DateToMilliseconds = DateToMilliseconds;
        /*==================================================================
         */
        var MillisecondsToDate = /** @class */ (function (_super) {
            __extends(MillisecondsToDate, _super);
            function MillisecondsToDate() {
                return _super !== null && _super.apply(this, arguments) || this;
            }
            MillisecondsToDate.prototype.onNext = function (value) {
                this.sendNext(new Date(value));
            };
            return MillisecondsToDate;
        }(Extension));
        reactive.MillisecondsToDate = MillisecondsToDate;
        /*==================================================================
         */
        var Offset = /** @class */ (function (_super) {
            __extends(Offset, _super);
            function Offset(dx, dy) {
                var _this = _super.call(this) || this;
                _this.dx = dx;
                _this.dy = dy;
                return _this;
            }
            Offset.prototype.onNext = function (value) {
                this.sendNext({ x: value.x + this.dx, y: value.y + this.dy });
            };
            return Offset;
        }(Extension));
        reactive.Offset = Offset;
        /*==================================================================
         */
        var PointToString = /** @class */ (function (_super) {
            __extends(PointToString, _super);
            function PointToString() {
                return _super !== null && _super.apply(this, arguments) || this;
            }
            PointToString.prototype.onNext = function (value) {
                this.sendNext('(' + value.x + ', ' + value.y + ')');
            };
            return PointToString;
        }(Extension));
        reactive.PointToString = PointToString;
        /*==================================================================
         */
        var Or = /** @class */ (function (_super) {
            __extends(Or, _super);
            function Or(observables) {
                var _this = _super.call(this) || this;
                _this.values = [];
                _this.completed = 0;
                for (var i = 0, l = observables.length; i < l; ++i) {
                    observables[i].addObserver(new reactive.ProxyObserver(_this, _this.onNext, _this.onError, _this.onCompleted, i));
                }
                return _this;
            }
            Or.prototype.onNext = function (value, i) {
                this.values[i] = value;
                this.update();
            };
            Or.prototype.onError = function (value, i) {
                this.values[i] = undefined;
                this.update();
            };
            Or.prototype.onCompleted = function () {
                ++this.completed;
                if (this.completed == this.values.length) {
                    this.sendCompleted();
                }
            };
            Or.prototype.update = function () {
                var x = this.values[0];
                for (var i = 1, l = this.values.length; !x && i < l; ++i) {
                    if (this.values[i]) {
                        x = this.values[i];
                    }
                }
                if (x !== this.mine) {
                    this.mine = x;
                    this.sendNext(x);
                }
            };
            return Or;
        }(reactive.BasicObservable));
        reactive.Or = Or;
    })(reactive = hd.reactive || (hd.reactive = {}));
})(hd || (hd = {}));
/*####################################################################
 * This is an implementation of the Promises/A+ standard.  See
 *   http://promisesaplus.com/
 * The entire spec is implemented.  Additionally, I have added extra
 * functionality in response to needs of our particular library.
 * Extra features are:
 *
 * 1.) Promise doubles as an Observable.  It issues an onNext for each
 *     notification.  If promise is fulfilled, it issues an onNext
 *     followed by an onCompleted.  If promise is rejected, it issues
 *     an onError followed by an onCompleted.
 *
 * 2.) Promise has an observable event named "ondropped".  This event
 *     sends onNext any time the last dependency of a promise is
 *     removed --- i.e., whenever the promise has a single dependency,
 *     and "removeDependency" is called to remove it.  Notes:
 *
 *     - A dependency is different from an observer.  This event fires
 *       only when the last dependency is removed, regardless of
 *       whether or not there are observers.
 *
 *     - When a promise is settled, the event sends onCompleted.
 *
 *     - The value sent for onNext is the promise itself.
 *
 * 3.) Promise is an Observer<Promise>.  Any promise received is
 *     immediately removed as a dependency.  Thus, given two promises,
 *     p and q, the following code:
 *
 *         p.addDependency( q );
 *         q.ondropped.addObserver( p );
 *
 *     has the this effect: If p is settled, the result passes on to
 *     q.  If q ever fires an ondropped event, then it is removed as a
 *     dependency from p.
 */
var hd;
(function (hd) {
    var reactive;
    (function (reactive) {
        var u = hd.utility;
        reactive.PromisePriority = 2;
        reactive.DroppedPriority = 1;
        /*==================================================================
         * Private type associating a dependency with the promise for the
         * dependency's value.
         */
        var DependencyBinding = /** @class */ (function () {
            function DependencyBinding(object, fulfilled_cb, rejected_cb, progress_cb, id, promise) {
                this.object = object;
                this.fulfilled_cb = fulfilled_cb;
                this.rejected_cb = rejected_cb;
                this.progress_cb = progress_cb;
                this.id = id;
                this.promise = promise;
            }
            DependencyBinding.prototype.onFulfilled = function (value) {
                if (this.fulfilled_cb) {
                    callHandler(this.fulfilled_cb, // call this function
                    this.object, // on this object
                    value, // passing this value
                    this.id, // and this identifier
                    this.promise // result resolves this prmoise
                    );
                }
                else if (this.promise) {
                    this.promise.resolve(value); // Must assume T == U
                }
            };
            DependencyBinding.prototype.onRejected = function (reason) {
                if (this.rejected_cb) {
                    callHandler(this.rejected_cb, // call this function
                    this.object, // on this object
                    reason, // passing this value
                    this.id, // and this identifier
                    this.promise // result resolves this promise
                    );
                }
                else if (this.promise) {
                    this.promise.reject(reason);
                }
            };
            DependencyBinding.prototype.onProgress = function (value) {
                if (this.progress_cb) {
                    callHandler(this.progress_cb, // call this function
                    this.object, // on this object
                    value, // passing this value
                    this.id, // and this identifier
                    undefined // result resolves this promise
                    );
                }
            };
            return DependencyBinding;
        }());
        /*==================================================================
         * Private enum for promise state
         */
        var State;
        (function (State) {
            State[State["Pending"] = 0] = "Pending";
            State[State["Fulfilled"] = 1] = "Fulfilled";
            State[State["Rejected"] = 2] = "Rejected";
        })(State = reactive.State || (reactive.State = {}));
        var Usage;
        (function (Usage) {
            Usage[Usage["Unknown"] = 0] = "Unknown";
            Usage[Usage["Used"] = 1] = "Used";
            Usage[Usage["Unused"] = 2] = "Unused";
            Usage[Usage["Delayed"] = 3] = "Delayed";
        })(Usage = reactive.Usage || (reactive.Usage = {}));
        ;
        /*==================================================================
         * Implementation of Promises/A+.
         */
        var Promise = /** @class */ (function (_super) {
            __extends(Promise, _super);
            /*----------------------------------------------------------------
             * Option to create promise already fulfilled.
             */
            function Promise(value) {
                var _this = _super.call(this) || this;
                // State of the promise
                _this.state = State.Pending;
                // Whether this promise has been resolved with another promise
                _this.open = true;
                // Promise usage
                _this.usage = new reactive.BasicSignal(Usage.Unknown);
                // Event fired when promise loses all observers
                _this.ondropped = new reactive.BasicObservable();
                if (arguments.length > 0) {
                    _this.resolve(value);
                }
                return _this;
            }
            /*----------------------------------------------------------------
             * State inspection methods.
             */
            Promise.prototype.isFulfilled = function () {
                return this.state === State.Fulfilled;
            };
            Promise.prototype.isRejected = function () {
                return this.state === State.Rejected;
            };
            Promise.prototype.isPending = function () {
                return this.state === State.Pending;
            };
            Promise.prototype.isSettled = function () {
                return this.state !== State.Pending;
            };
            Promise.prototype.hasValue = function () {
                return 'value' in this;
            };
            Promise.prototype.inspect = function () {
                if (this.state === State.Fulfilled) {
                    return { state: 'fulfilled', value: this.value };
                }
                else if (this.state === State.Rejected) {
                    return { state: 'rejected', reason: this.reason };
                }
                else {
                    if ('value' in this) {
                        return { state: 'pending', value: this.value };
                    }
                    else {
                        return { state: 'pending' };
                    }
                }
            };
            Promise.prototype.addDependency = function (object, onFulfilled, onRejected, onProgress, id) {
                var dependency;
                if (arguments.length == 1) {
                    dependency = object;
                }
                else {
                    dependency =
                        new DependencyBinding(object, onFulfilled, onRejected, onProgress, id);
                }
                if (this.state !== State.Pending) {
                    u.schedule(reactive.PromisePriority, this.dischargeDependency, this, dependency);
                }
                else {
                    // Debug info
                    if (reactive.plogger && this.dependencies.length == 0) {
                        reactive.plogger.nowHasDependencies(this);
                    }
                    if (this.hasOwnProperty('dependencies')) {
                        this.dependencies.push(dependency);
                    }
                    else {
                        // copy on write
                        this.dependencies = [dependency];
                    }
                }
                if (this.usage.get() === Usage.Unknown) {
                    this.usage.set(Usage.Used);
                }
                return dependency;
            };
            Promise.prototype.bindDependency = function (promise, object, onFulfilled, onRejected, onProgress, id) {
                var binding;
                if (arguments.length < 3) {
                    var d = object;
                    binding = this.addDependency(object, d.onFulfilled, d.onRejected, d.onProgress);
                }
                else {
                    binding = this.addDependency(object, onFulfilled, onRejected, onProgress, id);
                }
                binding.promise = promise;
                return binding;
            };
            /*----------------------------------------------------------------
             * Unsubscribe a dependency.
             *
             * Note that this implementation does not watch for duplicate
             * dependencies; this will only remove a single subscription of
             * the dependency.  If a dependency has been added multiple times
             * then it must be removed multiple times.
             */
            Promise.prototype.removeDependency = function (object) {
                var found = false;
                for (var i = this.dependencies.length - 1; i >= 0; --i) {
                    var d = this.dependencies[i];
                    if (d === object ||
                        (d instanceof DependencyBinding &&
                            d.object === object)) {
                        this.dependencies.splice(i, 1);
                        // Check to see if this promise is dropped
                        if (this.dependencies.length == 0) {
                            // Debug info
                            if (reactive.plogger) {
                                reactive.plogger.lostAllDependencies(this);
                            }
                            u.schedule(reactive.DroppedPriority, this.ondropped.sendNext, this.ondropped, this);
                        }
                        found = true;
                    }
                }
                return found;
            };
            /*----------------------------------------------------------------
             */
            Promise.prototype.hasDependencies = function () {
                return this.dependencies.length > 0;
            };
            Promise.prototype.resolve = function (value) {
                if (this.open) {
                    this.resolveFirst(value);
                }
            };
            Promise.prototype.resolveFirst = function (value) {
                if (this.state === State.Pending) {
                    if (value === this) {
                        this.reject(new TypeError("Attempted to resolve a promise with itself"));
                    }
                    else {
                        if (value instanceof Promise) {
                            // When other promise resolves, we resolve
                            this.open = false;
                            value.addDependency(this);
                        }
                        else if ((typeof value === 'object' || typeof value === 'function') &&
                            typeof value.then === 'function') {
                            // Provides compatability with other promise implementations
                            this.open = false;
                            this.resolveThen(value);
                        }
                        else {
                            // A value fulfills the promise
                            this.fulfill(value);
                        }
                    }
                }
            };
            /*----------------------------------------------------------------
             * Resolves a value that is not a promise but does have a "then"
             * method.
             *
             * Basically this is a wrapper to make sure the "then" method
             * behaves like a promise: only resolves/rejects once; doesn't
             * throw exceptions
             */
            Promise.prototype.resolveThen = function (value) {
                var pending = true;
                var This = this;
                try {
                    value.then(function (value) {
                        if (pending) {
                            pending = false;
                            This.resolve(value);
                        }
                    }, function (reason) {
                        if (pending) {
                            pending = false;
                            This.reject(reason);
                        }
                    }, function (value) {
                        if (pending) {
                            This.notify(value);
                        }
                    });
                }
                catch (e) {
                    if (pending) {
                        pending = false;
                        This.reject(e);
                    }
                }
            };
            /*----------------------------------------------------------------
             * Called when resolving concluded we really have a value and
             * not a further promise.
             */
            Promise.prototype.fulfill = function (value) {
                this.state = State.Fulfilled;
                this.value = value;
                // Debug info
                if (reactive.plogger) {
                    reactive.plogger.isSettled(this);
                }
                // Notify dependencies
                var dependencies = this.dependencies;
                u.schedule(reactive.PromisePriority, function () {
                    for (var i = 0, l = dependencies.length; i < l; ++i) {
                        dependencies[i].onFulfilled(value);
                    }
                }, this);
                // Notify observers
                this.sendNext(value);
                // Clean up
                delete this.dependencies;
                this.sendCompleted();
                delete this.observers;
                u.schedule(reactive.DroppedPriority, this.ondropped.sendCompleted, this.ondropped);
            };
            /*----------------------------------------------------------------
             * Rejects the promise with specified reason.
             */
            Promise.prototype.reject = function (reason) {
                if (this.open) {
                    this.rejectFirst(reason);
                }
            };
            Promise.prototype.rejectFirst = function (reason) {
                if (this.state === State.Pending) {
                    this.state = State.Rejected;
                    this.reason = reason;
                    delete this.value;
                    // Debug info
                    if (reactive.plogger) {
                        reactive.plogger.isSettled(this);
                    }
                    // Notify dependencies
                    var dependencies = this.dependencies;
                    u.schedule(reactive.PromisePriority, function () {
                        for (var i = 0, l = dependencies.length; i < l; ++i) {
                            dependencies[i].onRejected(reason);
                        }
                    }, this);
                    // Notify observers
                    this.sendError(reason);
                    // Clean up
                    delete this.dependencies;
                    this.sendCompleted();
                    u.schedule(reactive.DroppedPriority, this.ondropped.sendCompleted, this.ondropped);
                }
            };
            /*----------------------------------------------------------------
             * Called once promise has been resolved to notify dependency.
             */
            Promise.prototype.dischargeDependency = function (binding) {
                if (this.state === State.Fulfilled) {
                    binding.onFulfilled(this.value);
                }
                else if (this.state === State.Rejected) {
                    binding.onRejected(this.reason);
                }
            };
            /*----------------------------------------------------------------
             * Sends a progress value to all dependencies.
             *
             * This is basically an event: if you're not subscribed when it
             * fires then you just don't get it.
             */
            Promise.prototype.notify = function (value) {
                if (this.open) {
                    this.notifyFirst(value);
                }
            };
            Promise.prototype.notifyFirst = function (value) {
                if (this.state == State.Pending) {
                    this.value = value;
                    // Notify dependencies
                    var dependencies = this.dependencies;
                    u.schedule(reactive.PromisePriority, function () {
                        for (var i = 0, l = dependencies.length; i < l; ++i) {
                            dependencies[i].onProgress(value);
                        }
                    }, this);
                    // Notify observers
                    this.sendNext(value);
                }
            };
            Promise.prototype.then = function (onFulfilled, onRejected, onProgress) {
                if (typeof onFulfilled !== 'function') {
                    onFulfilled = null;
                }
                if (typeof onRejected !== 'function') {
                    onRejected = null;
                }
                if (typeof onProgress !== 'function') {
                    onProgress = null;
                }
                if (onFulfilled || onRejected || onProgress) {
                    var p = new Promise();
                    this.bindDependency(p, null, onFulfilled, onRejected, onProgress);
                    return p;
                }
                else {
                    return this;
                }
            };
            Promise.prototype.catch = function (onRejected) {
                if (typeof onRejected === 'function') {
                    var p = new Promise();
                    this.bindDependency(p, null, null, onRejected, null);
                    return p;
                }
                else {
                    return this;
                }
            };
            /*----------------------------------------------------------------
             * Chaining based approach to dependencies.
             *
             * Creates a new dependency with the specified onProgress
             * callback; returns a promise for that dependency's value.
             */
            Promise.prototype.progress = function (onProgress) {
                if (typeof onProgress === 'function') {
                    var p = new Promise();
                    this.bindDependency(p, null, null, null, onProgress);
                    return p;
                }
                else {
                    return this;
                }
            };
            Promise.prototype.onError = function () { };
            Promise.prototype.onCompleted = function () { };
            Promise.all = function () {
                if (arguments.length) {
                    var final = new Promise();
                    var values = [];
                    var count = 0;
                    var length = arguments.length;
                    var update = function (i, v) {
                        values[i] = v;
                        if (++count == length) {
                            final.resolve(values);
                        }
                    };
                    for (var i = 0; i < arguments.length; ++i) {
                        signup(arguments[i], i, update);
                    }
                }
                else {
                    var final = new Promise([]);
                }
                return final;
            };
            return Promise;
        }(reactive.BasicObservable));
        reactive.Promise = Promise;
        Promise.prototype.onFulfilled = Promise.prototype.resolveFirst;
        Promise.prototype.onRejected = Promise.prototype.rejectFirst;
        Promise.prototype.onProgress = Promise.prototype.notifyFirst;
        Promise.prototype.onNext = Promise.prototype.removeDependency;
        // Inherited empty dependency list
        Promise.prototype['dependencies'] = [];
        /*================================================================--
         * Add a callback to f to promise p.
         */
        function signup(p, i, f) {
            p.then(function (v) {
                f(i, v);
            });
        }
        /*==================================================================
         * Function to call a handler on a dependency.
         */
        function callHandler(handler, dependency, value, id, promise) {
            try {
                var result = handler.call(dependency, value, id);
                if (promise) {
                    promise.resolve(result);
                }
            }
            catch (e) {
                console.warn(e);
                if (promise) {
                    promise.reject(e);
                }
            }
        }
    })(reactive = hd.reactive || (hd.reactive = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var reactive;
    (function (reactive) {
        var AccumulatingPromise = /** @class */ (function (_super) {
            __extends(AccumulatingPromise, _super);
            function AccumulatingPromise(value) {
                var _this = _super.call(this) || this;
                _this.accumState = 0 /* Pending */;
                _this.accumValue = undefined;
                _this.accumReason = undefined;
                _this.timeout = null;
                _this.ontimeout = _this.ontimeout.bind(_this);
                if (arguments.length > 0) {
                    _this.accumState = 1 /* Fulfill */;
                    _this.accumValue = value;
                }
                return _this;
            }
            AccumulatingPromise.prototype.setDelay = function (delay) {
                if (this.timeout) {
                    clearTimeout(this.timeout);
                    this.timeout = null;
                }
                this.delay = delay;
                if (this.accumState === 1 /* Fulfill */ ||
                    this.accumState === 2 /* Reject */) {
                    this.timeout = setTimeout(this.ontimeout, delay);
                }
            };
            AccumulatingPromise.prototype.ontimeout = function () {
                this.timeout = null;
                this.settle();
            };
            AccumulatingPromise.prototype.settle = function () {
                if (this.timeout) {
                    clearTimeout(this.timeout);
                    this.timeout = null;
                }
                if (this.accumState === 1 /* Fulfill */) {
                    this.resolve(this.accumValue);
                    this.accumValue = undefined;
                }
                else if (this.accumState === 2 /* Reject */) {
                    this.reject(this.accumReason);
                }
                this.accumState = 3 /* Settled */;
            };
            AccumulatingPromise.prototype.updateResolve = function (value) {
                if (this.accumState === 3 /* Settled */) {
                    this.resolve(value);
                }
                else {
                    if (this.timeout) {
                        clearTimeout(this.timeout);
                        this.timeout = null;
                    }
                    this.accumState = 1 /* Fulfill */;
                    this.accumValue = value;
                    this.accumReason = undefined;
                    if (this.delay) {
                        this.timeout = setTimeout(this.ontimeout, this.delay);
                    }
                }
            };
            AccumulatingPromise.prototype.updateReject = function (reason) {
                if (this.accumState === 3 /* Settled */) {
                    this.reject(reason);
                }
                else {
                    if (this.timeout) {
                        clearTimeout(this.timeout);
                        this.timeout = null;
                    }
                    this.accumState = 2 /* Reject */;
                    this.accumReason = reason;
                    this.accumValue = undefined;
                    if (this.delay) {
                        this.timeout = setTimeout(this.ontimeout, this.delay);
                    }
                }
            };
            return AccumulatingPromise;
        }(reactive.Promise));
        reactive.AccumulatingPromise = AccumulatingPromise;
    })(reactive = hd.reactive || (hd.reactive = {}));
})(hd || (hd = {}));
/*####################################################################
* Defines PromiseLadder, which converts a sequence of promises into
* a single observable.
*/
var hd;
(function (hd) {
    var reactive;
    (function (reactive) {
        /*==================================================================
         * Converts a sequence of promises into a single observable by
         * firing on fulfilled promises only if no more recent promises
         * have fulfilled.  So, once a promise is fulfilled, any older
         * promises are dropped.
         *
         * Invariant:  entries[0].state == 'fulfilled' || entries[0].state == 'rejected'
         */
        var PromiseLadder = /** @class */ (function (_super) {
            __extends(PromiseLadder, _super);
            /*----------------------------------------------------------------
             * Initialize
             */
            function PromiseLadder() {
                var _this = _super.call(this) || this;
                var p = new reactive.Promise();
                if (reactive.plogger) {
                    reactive.plogger.register(p, 'ladder', 'ladder initialization');
                }
                p.resolve(undefined);
                _this.entries = [{ promise: p,
                        state: 'fulfilled',
                        value: undefined
                    }
                ];
                return _this;
            }
            /*----------------------------------------------------------------
             * Is there any possibility the value will change (assuming no
             * further promises are added)?
             */
            PromiseLadder.prototype.isSettled = function () {
                var i = this.entries.length - 1;
                while (this.entries[i].state === 'failed') {
                    --i;
                }
                return this.entries[i].state !== 'pending';
            };
            /*----------------------------------------------------------------
             */
            PromiseLadder.prototype.currentFailed = function () {
                return this.entries[this.entries.length - 1].state === 'failed';
            };
            /*----------------------------------------------------------------
             * Get the most recent promise on the ladder.
             */
            PromiseLadder.prototype.getCurrentPromise = function () {
                return this.entries[this.entries.length - 1].promise;
            };
            /*----------------------------------------------------------------
             * Get a promise forwarded from most recent promise
             */
            PromiseLadder.prototype.forwardPromise = function (forward) {
                var last = this.entries.length - 1;
                // Try to forward
                if (!this.tryToForward(forward, last)) {
                    forward.ondropped.addObserver(this, this.onForwardDropped, null, null, this.entries[last].promise);
                    // If fails, add to forward list
                    if (!this.entries[last].forwards) {
                        this.entries[last].forwards = [forward];
                    }
                    else {
                        this.entries[last].forwards.push(forward);
                    }
                }
                return forward;
            };
            /*----------------------------------------------------------------
             */
            PromiseLadder.prototype.findPromiseIndex = function (p) {
                for (var i = 0, l = this.entries.length; i < l; ++i) {
                    if (this.entries[i].promise === p) {
                        return i;
                    }
                }
                return -1;
            };
            /*----------------------------------------------------------------
             * Find most recent entry to produce results.
             * "Results" means either fulfilled, rejected, or pending with a
             * notification.
             */
            PromiseLadder.prototype.getMostRecent = function () {
                for (var i = this.entries.length - 1; i >= 0; --i) {
                    var entry = this.entries[i];
                    var state = entry.state;
                    if (state === 'fulfilled' ||
                        state === 'rejected' ||
                        (state === 'pending' && 'value' in entry)) {
                        break;
                    }
                }
                return i;
            };
            /*----------------------------------------------------------------
             * Add new promises to the ladder.
             */
            PromiseLadder.prototype.addPromise = function (promise) {
                // Add promise
                this.entries.push({ promise: promise, state: 'pending' });
                // Subscribe
                promise.addDependency(this, this.onPromiseFulfilled, this.onPromiseRejected, this.onPromiseProgress, promise);
            };
            /*----------------------------------------------------------------
             * Do work for fulfilled promise
             */
            PromiseLadder.prototype.onPromiseFulfilled = function (value, promise) {
                var i = this.findPromiseIndex(promise);
                if (i >= 0) {
                    this.entries[i].state = 'fulfilled';
                    this.entries[i].value = value;
                    if (this.getMostRecent() == i) {
                        this.sendNext(value);
                    }
                    this.updateForwardsStartingFrom(i);
                }
            };
            /*----------------------------------------------------------------
             * Do work for rejected promise
             */
            PromiseLadder.prototype.onPromiseRejected = function (reason, promise) {
                var i = this.findPromiseIndex(promise);
                if (i >= 0) {
                    // Differentiate between "rejected" and "failed"
                    if (reason === null || reason === undefined) {
                        this.entries[i].state = 'failed';
                        // Special case 1:  this promise previously produced a notification;
                        //   thus, we need to fall-back to the previous answer
                        var mostRecent = this.getMostRecent();
                        if ('value' in this.entries[i] && mostRecent < i) {
                            var entry = this.entries[mostRecent];
                            if (entry.state === 'rejected') {
                                this.sendNext(entry.reason);
                            }
                            else {
                                this.sendNext(entry.value);
                            }
                        }
                        // Special case 2:  this promise is at the top of the ladder;
                        //   we need to let the variable know so that it can be marked stale
                        else if (i == this.entries.length - 1) {
                            this.sendError(null);
                        }
                    }
                    else {
                        this.entries[i].state = 'rejected';
                        this.entries[i].reason = reason;
                        if (this.getMostRecent() == i) {
                            this.sendError(reason);
                        }
                    }
                    this.updateForwardsStartingFrom(i);
                }
            };
            /*----------------------------------------------------------------
             * Do work for promise notification
             */
            PromiseLadder.prototype.onPromiseProgress = function (value, promise) {
                var i = this.findPromiseIndex(promise);
                if (i >= 0) {
                    this.entries[i].value = value;
                    if (this.getMostRecent() == i) {
                        this.sendNext(value);
                    }
                    this.updateForwardsStartingFrom(i);
                }
            };
            /*----------------------------------------------------------------
             * Forwarded promise no longer needed
             */
            PromiseLadder.prototype.onForwardDropped = function (forward, promise) {
                var i = this.findPromiseIndex(promise);
                var forwards = this.entries[i].forwards;
                if (i >= 0 && forwards) {
                    var j = forwards.indexOf(forward);
                    if (j >= 0) {
                        if (forwards.length == 1) {
                            this.entries[i].forwards = undefined;
                            this.dropUnneededPromises();
                        }
                        else {
                            forwards.splice(j, 1);
                        }
                    }
                }
            };
            /*----------------------------------------------------------------
             * Try to perform all forwards starting at index i and continuing
             * up as long as promises were failed.
             */
            PromiseLadder.prototype.updateForwardsStartingFrom = function (i) {
                // Try to perform all forwards for this promise
                this.tryToForwardList(i, i);
                // Try to perform any forwards for more recent promises that have failed
                for (var j = i + 1, l = this.entries.length; j < l && this.entries[j].state === 'failed'; ++j) {
                    this.tryToForwardList(j, i);
                }
                // Clean up
                this.dropUnneededPromises();
            };
            /*----------------------------------------------------------------
             * Try to forward each promise on the list.  Returns list of
             * promises which could /not/ be forwarded.
             */
            PromiseLadder.prototype.tryToForwardList = function (target, start) {
                var forwards = this.entries[target].forwards;
                if (forwards) {
                    var remaining = [];
                    for (var i = 0, l = forwards.length; i < l; ++i) {
                        if (!this.tryToForward(forwards[i], start)) {
                            remaining.push(forwards[i]);
                        }
                    }
                    this.entries[target].forwards = remaining.length == 0 ? undefined : remaining;
                }
            };
            /*----------------------------------------------------------------
             * Try to settle forward using this.promises[i].  Returns true iff
             * forward was settled.
             */
            PromiseLadder.prototype.tryToForward = function (forward, i) {
                var entry = this.entries[i];
                if (entry.state === 'fulfilled') {
                    forward.resolve(entry.value);
                    return true;
                }
                if (entry.state === 'rejected') {
                    forward.reject(entry.reason);
                    return true;
                }
                if (entry.state === 'failed') {
                    return this.tryToForward(forward, i - 1);
                }
                if (entry.state === 'pending' && 'value' in entry) {
                    forward.notify(entry.value);
                }
                return false;
            };
            /*----------------------------------------------------------------
             * Drop all promises which are settled or which can no longer
             * affect the ladder value and have no forwards.
             */
            PromiseLadder.prototype.dropUnneededPromises = function () {
                var last = this.entries.length - 1;
                var removeAnswered = false;
                var removePending = false;
                for (var i = last; i >= 0; --i) {
                    var entry = this.entries[i];
                    var state = entry.state;
                    if (state === 'fulfilled' || state === 'rejected') {
                        if (removeAnswered) {
                            this.entries.splice(i, 1);
                        }
                        removeAnswered = true;
                        removePending = true;
                    }
                    else {
                        if (entry.forwards) {
                            removeAnswered = false;
                        }
                        else if (state === 'failed') {
                            if (i != last) {
                                this.entries.splice(i, 1);
                            }
                        }
                        else if (state === 'pending') {
                            if (removePending) {
                                this.entries.splice(i, 1);
                            }
                        }
                    }
                }
            };
            return PromiseLadder;
        }(reactive.BasicObservable));
        reactive.PromiseLadder = PromiseLadder;
    })(reactive = hd.reactive || (hd.reactive = {}));
})(hd || (hd = {}));
/*####################################################################
 * Lifting function over values to function over promises.
 */
var hd;
(function (hd) {
    var reactive;
    (function (reactive) {
        var r = hd.reactive;
        /*==================================================================
         * The actual lifting function.
         */
        function liftFunction(fn, mask) {
            if (mask === void 0) { mask = null; }
            return function () {
                var parameters = [];
                for (var _i = 0; _i < arguments.length; _i++) {
                    parameters[_i] = arguments[_i];
                }
                var invocation = new LiftedFunctionInvocation(fn, parameters, mask);
                return invocation.output;
            };
        }
        reactive.liftFunction = liftFunction;
        // unique value used to indicate that the corresponding promise has
        //   never produced a value (including progress notifications)
        var noValue = {};
        /*==================================================================
         * A lifted function invocation.
         */
        var LiftedFunctionInvocation = /** @class */ (function () {
            /*----------------------------------------------------------------
             * Subscribe to all input promises
             */
            function LiftedFunctionInvocation(fn, inputs, mask) {
                if (mask === void 0) { mask = null; }
                // List of promises we have subscribed to
                this.promises = [];
                // How many promises are currently pending
                this.pending = 0;
                // How many promises have never produced a value (pending + !progress)
                this.noValue = 0;
                // The output of the function
                this.output = new r.Promise();
                this.fn = fn;
                this.inputs = inputs;
                this.params = this.genParams(inputs, mask, []);
                this.output.ondropped.addObserver(this);
                if (this.pending == 0) {
                    this.invoke();
                }
            }
            /*----------------------------------------------------------------
             * Convert input array to parameter array -- i.e., replacing
             * promises with their value.
             */
            LiftedFunctionInvocation.prototype.genParams = function (inputs, mask, place) {
                var params = [];
                for (var i = 0, l = inputs.length; i < l; ++i) {
                    var input = inputs[i];
                    if (Array.isArray(input)) {
                        params[i] = this.genParams(input, Array.isArray(mask) ? mask[i] : null, place.concat([i]));
                    }
                    else if (input instanceof r.Promise) {
                        if (Array.isArray(mask) && mask[i]) {
                            params[i] = input;
                        }
                        else {
                            params[i] = noValue;
                            input.addDependency(this, this.onParamFulfilled, this.onParamRejected, this.onParamProgress, place.concat([i]));
                            this.promises.push(input);
                            ++this.pending;
                            ++this.noValue;
                        }
                    }
                    else {
                        params[i] = input;
                    }
                }
                return params;
            };
            /*----------------------------------------------------------------
             * Inserts the value of a resolved promise into the parameter
             * multi-array.
             */
            LiftedFunctionInvocation.prototype.resolveParam = function (value, params, place, i) {
                var idx = place[i];
                if (i == place.length - 1) {
                    if (params[idx] === noValue) {
                        --this.noValue;
                    }
                    params[idx] = value;
                }
                else {
                    this.resolveParam(value, params[idx], place, i + 1);
                }
            };
            /*----------------------------------------------------------------
             * Called when promise resolves
             */
            LiftedFunctionInvocation.prototype.onParamFulfilled = function (value, place) {
                this.resolveParam(value, this.params, place, 0);
                --this.pending;
                if (this.noValue == 0) {
                    this.invoke();
                }
            };
            /*----------------------------------------------------------------
             * If any parameter fails then we fail
             */
            LiftedFunctionInvocation.prototype.onParamRejected = function (reason) {
                this.output.reject(null);
                this.promises.forEach(function (p) {
                    p.removeDependency(this);
                }, this);
            };
            /*----------------------------------------------------------------
             * Set the corresponding value; notify
             */
            LiftedFunctionInvocation.prototype.onParamProgress = function (value, place) {
                this.resolveParam(value, this.params, place, 0);
                if (this.noValue == 0) {
                    this.invoke();
                }
            };
            /*----------------------------------------------------------------
             */
            LiftedFunctionInvocation.prototype.invoke = function () {
                try {
                    var result = this.fn.apply(null, this.params);
                    if (this.pending == 0) {
                        this.output.resolve(result);
                    }
                    else {
                        this.output.notify(result);
                    }
                }
                catch (e) {
                    console.error(e);
                    this.output.reject(e);
                }
            };
            /*----------------------------------------------------------------
             */
            LiftedFunctionInvocation.prototype.onNext = function (p) {
                if (p === this.output) {
                    this.promises.forEach(function (p) {
                        p.removeDependency(this);
                    }, this);
                }
            };
            LiftedFunctionInvocation.prototype.onError = function () { };
            LiftedFunctionInvocation.prototype.onCompleted = function () { };
            return LiftedFunctionInvocation;
        }());
    })(reactive = hd.reactive || (hd.reactive = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var reactive;
    (function (reactive) {
        var u = hd.utility;
        var PromiseLogger = /** @class */ (function () {
            function PromiseLogger() {
                this.count = 1;
                this.outstanding = [];
            }
            PromiseLogger.prototype.register = function (p, tag, reason) {
                if (p.id) {
                    console.error('Logging problem: attempt to re-register promise ' + p.id);
                }
                else {
                    p.id = tag + '#' + this.count++;
                    // if (reason) {
                    //   console.log( 'Promise created for ' + reason + ': ' + p.id );
                    // }
                    // else {
                    //   console.log( 'Promise created:  ' + p.id );
                    // }
                }
            };
            PromiseLogger.prototype.checkId = function (p) {
                if (!p.id) {
                    this.register(p, 'unknown');
                }
            };
            PromiseLogger.prototype.nowHasDependencies = function (p) {
                this.checkId(p);
                u.arraySet.add(this.outstanding, p);
                console.log('Promise expected: ' + p.id);
            };
            PromiseLogger.prototype.lostAllDependencies = function (p) {
                this.checkId(p);
                u.arraySet.remove(this.outstanding, p);
                console.log('Promise dropped:  ' + p.id);
            };
            PromiseLogger.prototype.isSettled = function (p) {
                this.checkId(p);
                u.arraySet.remove(this.outstanding, p);
                console.log('Promise settled:  ' + p.id);
            };
            PromiseLogger.prototype.report = function () {
                if (this.outstanding.length) {
                    console.log('Outstanding: ' +
                        this.outstanding.map(function (p) {
                            return p.id;
                        }).join(', '));
                }
                else {
                    console.log('No outstanding promises');
                }
                return this.outstanding;
            };
            PromiseLogger.prototype.activation = function (inputs, outputs) {
                console.log('Activation: ' +
                    inputs.map(u.getId).join(', ') +
                    ' -> ' +
                    outputs.map(u.getId).join(', '));
            };
            return PromiseLogger;
        }());
        reactive.PromiseLogger = PromiseLogger;
    })(reactive = hd.reactive || (hd.reactive = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var r = hd.reactive;
    hd.ProxyObserver = r.ProxyObserver;
    hd.BasicObservable = r.BasicObservable;
    hd.Promise = r.Promise;
    hd.Extension = r.Extension;
    hd.liftFunction = r.liftFunction;
    /*==================================================================
     * Enablement functions
     */
    function markUsed(p) {
        p.usage.set(r.Usage.Used);
    }
    hd.markUsed = markUsed;
    function markUnused(p) {
        p.usage.set(r.Usage.Unused);
    }
    hd.markUnused = markUnused;
    function markDelayed(p) {
        p.usage.set(r.Usage.Delayed);
    }
    hd.markDelayed = markDelayed;
})(hd || (hd = {}));
//
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (b.hasOwnProperty(p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
/*####################################################################
 * GraphWalker and DigraphWalker for performing depth-first
 * traversals.
 *
 * This was written for bipartite graphs, so traversal methods include
 * the option to visit every node or every other node.
 */
var hd;
(function (hd) {
    var graph;
    (function (graph) {
        /*==================================================================
         * Base class for walkers.  Has the actual traversal methods --
         * subclasses just provide traversal parameters.
         */
        var DfsWalker = /** @class */ (function () {
            /*----------------------------------------------------------------
             * Special method for collected array
             */
            function DfsWalker() {
                // Every node visited by the walker
                this.visited = {};
                // Nodes collected by the walker
                this.collected = [];
                var walker = this;
                this.collected.and = function () { return walker; };
            }
            /*----------------------------------------------------------------
             * Get all nodes collected by the walker.
             */
            DfsWalker.prototype.result = function () {
                return this.collected;
            };
            DfsWalker.prototype.collect = function (visit, starting) {
                if (Array.isArray(starting)) {
                    starting.forEach(function (n) {
                        this.collectRec(visit, n);
                    }, this);
                }
                else {
                    this.collectRec(visit, starting);
                }
                return this.collected;
            };
            /*----------------------------------------------------------------
             * The actual recursive traversal function.
             */
            DfsWalker.prototype.collectRec = function (visit, nid) {
                if (this.visited[nid]) {
                    return;
                }
                this.visited[nid] = true;
                if (visit >= 0) {
                    // We know this is unique, so just push
                    this.collected.push(nid);
                }
                var toEdges = this.edges[nid];
                for (var n2id in toEdges) {
                    this.collectRec(-visit, n2id);
                }
            };
            return DfsWalker;
        }());
        graph.DfsWalker = DfsWalker;
    })(graph = hd.graph || (hd.graph = {}));
})(hd || (hd = {}));
/*####################################################################
 * The hd.graph.Digraph class.
 */
var hd;
(function (hd) {
    var graph;
    (function (graph_1) {
        /*==================================================================
         * A directed graph.
         */
        var Digraph = /** @class */ (function () {
            function Digraph() {
                // Labels for nodes which have one
                this.labels = {};
                // All incoming edges
                this.ins = {};
                // All outgoing edges
                this.outs = {};
            }
            /*----------------------------------------------------------------
             * Is node in the graph?
             */
            Digraph.prototype.hasNode = function (n) {
                return n in this.outs;
            };
            /*----------------------------------------------------------------
             * Is edge in the graph?
             */
            Digraph.prototype.hasEdge = function (from, to) {
                return this.hasNode(from) && this.outs[from][to];
            };
            /*----------------------------------------------------------------
             * Getter for node label.
             */
            Digraph.prototype.getLabel = function (n) {
                return this.labels[n];
            };
            /*----------------------------------------------------------------
             * Setter for node label.
             */
            Digraph.prototype.setLabel = function (n, label) {
                if (label === undefined) {
                    delete this.labels[n];
                }
                else {
                    this.labels[n] = label;
                }
            };
            /*----------------------------------------------------------------
             * Add node to the graph.  Has no effect if node already in graph.
             */
            Digraph.prototype.addNode = function (n, label) {
                if (!this.outs[n]) {
                    if (label !== undefined) {
                        this.labels[n] = label;
                    }
                    this.outs[n] = {};
                    this.ins[n] = {};
                }
            };
            /*----------------------------------------------------------------
             * Add edge to the graph.
             */
            Digraph.prototype.addEdge = function (from, to) {
                this.addNode(from);
                this.addNode(to);
                this.outs[from][to] = true;
                this.ins[to][from] = true;
            };
            /*----------------------------------------------------------------
             * Curried version of addEdge - returns a function which can be
             * used to add edges going to specified node.
             */
            Digraph.prototype.addEdgeTo = function (to) {
                var graph = this;
                return function (from) {
                    graph.addEdge(from, to);
                };
            };
            /*----------------------------------------------------------------
             * Curried version of addEdge - returns a function which can be
             * used to add edges coming from specified node.
             */
            Digraph.prototype.addEdgeFrom = function (from) {
                var graph = this;
                return function (to) {
                    graph.addEdge(from, to);
                };
            };
            /*----------------------------------------------------------------
             * Remove node from the graph.
             */
            Digraph.prototype.removeNode = function (n) {
                delete this.labels[n];
                for (var to in this.outs[n]) {
                    delete this.ins[to][n];
                }
                for (var from in this.ins[n]) {
                    delete this.outs[from][n];
                }
                delete this.outs[n];
                delete this.ins[n];
            };
            /*----------------------------------------------------------------
             * Remove edge from the graph.
             */
            Digraph.prototype.removeEdge = function (from, to) {
                delete this.outs[from][to];
                delete this.ins[to][from];
            };
            /*----------------------------------------------------------------
             * All nodes in the graph.
             */
            Digraph.prototype.getNodes = function () {
                return Object.keys(this.outs);
            };
            /*----------------------------------------------------------------
             * Only nodes which have labels.
             */
            Digraph.prototype.getLabeledNodes = function () {
                return Object.keys(this.labels);
            };
            /*----------------------------------------------------------------
             * Only nodes which do not have labels.
             */
            Digraph.prototype.getUnlabeledNodes = function () {
                return Object.keys(this.outs).filter(function (id) {
                    return !(id in this.labels);
                });
            };
            /*----------------------------------------------------------------
             * All outgoing edges for one particular node.
             */
            Digraph.prototype.getOutsFor = function (n) {
                var ns = [];
                var outs = this.outs[n];
                for (var n2 in outs) {
                    if (outs[n2]) {
                        ns.push(n2);
                    }
                }
                return ns;
            };
            /*----------------------------------------------------------------
             * All incoming edges for one particular node.
             */
            Digraph.prototype.getInsFor = function (n) {
                var ns = [];
                var ins = this.ins[n];
                for (var n2 in ins) {
                    if (ins[n2]) {
                        ns.push(n2);
                    }
                }
                return ns;
            };
            return Digraph;
        }());
        graph_1.Digraph = Digraph;
        /*==================================================================
         * DfsWalker for a Graph.  Can walk over outgoing edges for
         * downstream traversal, or incoming edges for upstream traversal.
         */
        var DigraphWalker = /** @class */ (function (_super) {
            __extends(DigraphWalker, _super);
            /*----------------------------------------------------------------
             * Stores digraph so that it can access its incoming and outgoing
             * edges as needed.
             */
            function DigraphWalker(digraph) {
                var _this = _super.call(this) || this;
                _this.digraph = digraph;
                return _this;
            }
            DigraphWalker.prototype.nodesDownstream = function (starting) {
                this.edges = this.digraph.outs;
                return this.collect(0, starting);
            };
            DigraphWalker.prototype.nodesDownstreamSameType = function (starting) {
                this.edges = this.digraph.outs;
                return this.collect(1, starting);
            };
            DigraphWalker.prototype.nodesDownstreamOtherType = function (starting) {
                this.edges = this.digraph.outs;
                return this.collect(-1, starting);
            };
            DigraphWalker.prototype.nodesUpstream = function (starting) {
                this.edges = this.digraph.ins;
                return this.collect(0, starting);
            };
            DigraphWalker.prototype.nodesUpstreamSameType = function (starting) {
                this.edges = this.digraph.ins;
                return this.collect(1, starting);
            };
            DigraphWalker.prototype.nodesUpstreamOtherType = function (starting) {
                this.edges = this.digraph.ins;
                return this.collect(-1, starting);
            };
            return DigraphWalker;
        }(graph_1.DfsWalker));
        graph_1.DigraphWalker = DigraphWalker;
    })(graph = hd.graph || (hd.graph = {}));
})(hd || (hd = {}));
/*####################################################################
 * The ConstraintGraph interface and classes.
 *
 * A constraint graph is simply a wrapper around a directed graph.  It
 * provides a more controlled way to build up the graph, as well as
 * methods which perform the standard queries you'd expect to perform.
 * Wrapping the graph also provides the opportunity to provide caching
 * for common queries if desired.
 *
 * A constraint graph may be viewed as a hypergraph, in which the
 * variables are nodes and the methods are hyperedges going from the
 * input variables to the output variables.  Thus, the "addVariable"
 * function adds a node to the graph, and the "addMethod" function
 * adds a hyperedge to the graph.
 */
var hd;
(function (hd) {
    var graph;
    (function (graph) {
        var u = hd.utility;
        /*==================================================================
         * This implementation of ConstraintGraph does not do any caching.
         * All information is stored in the graph (constraint ids are
         * stored as labels for method nodes).  Queries are recaclulated
         * every time directly from the graph.
         */
        var NoCachingConstraintGraph = /** @class */ (function () {
            function NoCachingConstraintGraph() {
                // The directed graph
                this.graph = new graph.Digraph();
            }
            /*----------------------------------------------------------------
             * Given a list of methods, return a set of the corresponding
             * constraints.
             */
            NoCachingConstraintGraph.prototype.constraintsFor = function (mids) {
                var cids = {};
                mids.forEach(function (mid) {
                    cids[this.graph.getLabel(mid)] = true;
                }, this);
                return Object.keys(cids);
            };
            /*----------------------------------------------------------------
             * Add variable node to the graph.
             */
            NoCachingConstraintGraph.prototype.addVariable = function (vid) {
                this.graph.addNode(vid);
            };
            /*----------------------------------------------------------------
             * Add method node to the graph.
             * Also add edges which use this new node at the same time.
             * Precondition: all inputs/outputs have already been added to the graph
             */
            NoCachingConstraintGraph.prototype.addMethod = function (mid, cid, inputs, outputs) {
                this.graph.addNode(mid, cid);
                inputs.forEach(this.graph.addEdgeTo(mid));
                outputs.forEach(this.graph.addEdgeFrom(mid));
            };
            /*----------------------------------------------------------------
             * Remove specified variable (and all edges which use that node).
             * Precondition: any methods which use this as input/output have
             * already been removed from the graph
             */
            NoCachingConstraintGraph.prototype.removeVariable = function (vid) {
                this.graph.removeNode(vid);
            };
            /*----------------------------------------------------------------
             * Remove specified method (and all edges which use that node).
             */
            NoCachingConstraintGraph.prototype.removeMethod = function (mid) {
                var cid = this.constraintForMethod(mid);
                this.graph.removeNode(mid);
            };
            /*----------------------------------------------------------------
             * All variables contained in the constraint graph.
             */
            NoCachingConstraintGraph.prototype.variables = function () {
                return this.graph.getUnlabeledNodes();
            };
            /*----------------------------------------------------------------
             * All methods contained in the constraint graph.
             */
            NoCachingConstraintGraph.prototype.methods = function () {
                return this.graph.getLabeledNodes();
            };
            /*----------------------------------------------------------------
             * All constraints contained in the constraint graph.
             */
            NoCachingConstraintGraph.prototype.constraints = function () {
                return this.constraintsFor(this.graph.getLabeledNodes());
            };
            /*----------------------------------------------------------------
             */
            NoCachingConstraintGraph.prototype.contains = function (id) {
                return this.graph.hasNode(id);
            };
            /*----------------------------------------------------------------
             * All methods which use specified variable as input.
             */
            NoCachingConstraintGraph.prototype.methodsWhichInput = function (vid) {
                return this.graph.getOutsFor(vid);
            };
            /*----------------------------------------------------------------
             * All methods which use specified variable as output.
             */
            NoCachingConstraintGraph.prototype.methodsWhichOutput = function (vid) {
                return this.graph.getInsFor(vid);
            };
            /*----------------------------------------------------------------
             * All methods which use specified variable (whether input or
             * ouptut)
             */
            NoCachingConstraintGraph.prototype.methodsWhichUse = function (vid) {
                var ins = this.graph.getInsFor(vid);
                var outs = this.graph.getOutsFor(vid);
                return u.arraySet.union(ins, outs);
            };
            /*----------------------------------------------------------------
             * All constraints which have a method using the specified
             * variable as input.
             */
            NoCachingConstraintGraph.prototype.constraintsWhichInput = function (vid) {
                return this.constraintsFor(this.methodsWhichInput(vid));
            };
            /*----------------------------------------------------------------
             * All constraints which have a method using the specified
             * variable as output.
             */
            NoCachingConstraintGraph.prototype.constraintsWhichOutput = function (vid) {
                return this.constraintsFor(this.methodsWhichOutput(vid));
            };
            /*----------------------------------------------------------------
             * All constraints which use the specified variable.
             */
            NoCachingConstraintGraph.prototype.constraintsWhichUse = function (vid) {
                return this.constraintsFor(this.methodsWhichUse(vid));
            };
            /*----------------------------------------------------------------
             * The variables used as input for the specified method.
             */
            NoCachingConstraintGraph.prototype.inputsForMethod = function (mid) {
                return this.graph.getInsFor(mid);
            };
            /*----------------------------------------------------------------
             * The variables used as output for the specified method.
             */
            NoCachingConstraintGraph.prototype.outputsForMethod = function (mid) {
                return this.graph.getOutsFor(mid);
            };
            /*----------------------------------------------------------------
             * The variables used as either input or output for the specified
             * method.
             */
            NoCachingConstraintGraph.prototype.variablesForMethod = function (mid) {
                var ins = this.graph.getInsFor(mid);
                var outs = this.graph.getOutsFor(mid);
                return u.arraySet.union(ins, outs);
            };
            /*----------------------------------------------------------------
             * The constraint which specified method belongs to.
             */
            NoCachingConstraintGraph.prototype.constraintForMethod = function (mid) {
                return this.graph.getLabel(mid);
            };
            /*----------------------------------------------------------------
             * The variables used as input for at least one method in the
             * specified constraint.
             */
            NoCachingConstraintGraph.prototype.inputsForConstraint = function (cid) {
                var mids = this.methodsForConstraint(cid);
                var inputsList = mids.map(this.graph.getInsFor, this.graph);
                return inputsList.reduce(u.arraySet.union, []);
            };
            /*----------------------------------------------------------------
             * The variables used as output for at least one method in the
             * specified constraint.
             */
            NoCachingConstraintGraph.prototype.outputsForConstraint = function (cid) {
                var mids = this.methodsForConstraint(cid);
                var outputsList = mids.map(this.graph.getOutsFor, this.graph);
                return outputsList.reduce(u.arraySet.union, []);
            };
            /*----------------------------------------------------------------
             * The variables used by specified constraint.
             */
            NoCachingConstraintGraph.prototype.variablesForConstraint = function (cid) {
                var mids = this.methodsForConstraint(cid);
                var inputsList = mids.map(this.graph.getInsFor, this.graph);
                var outputsList = mids.map(this.graph.getOutsFor, this.graph);
                var inputs = inputsList.reduce(u.arraySet.union, []);
                var outputs = outputsList.reduce(u.arraySet.union, []);
                return u.arraySet.union(inputs, outputs);
            };
            /*----------------------------------------------------------------
             * The methods belonging to the specified constraint.
             */
            NoCachingConstraintGraph.prototype.methodsForConstraint = function (cid) {
                return this.methods().filter(function (mid) {
                    return this.graph.getLabel(mid) == cid;
                }, this);
            };
            return NoCachingConstraintGraph;
        }());
        graph.NoCachingConstraintGraph = NoCachingConstraintGraph;
        /*==================================================================
         * This implementation of ConstraintGraph expands on the simple
         * NoCachingConstraintGraph by keeping a set of all variable ids and
         * a map from constraint id to the methods in the constraint.
         * This speeds up the slowest queries from NoCachingConstraintGraph.
         */
        var CachingConstraintGraph = /** @class */ (function (_super) {
            __extends(CachingConstraintGraph, _super);
            function CachingConstraintGraph() {
                var _this = _super !== null && _super.apply(this, arguments) || this;
                // Keeps ids of all variables
                _this.vids = {};
                // Maps constraint ids to methods in the constraint
                _this.cids = {};
                return _this;
            }
            /*----------------------------------------------------------------
             * Add variable / update cache
             */
            CachingConstraintGraph.prototype.addVariable = function (vid) {
                _super.prototype.addVariable.call(this, vid);
                this.vids[vid] = true;
            };
            /*----------------------------------------------------------------
             * Add method / update cache
             */
            CachingConstraintGraph.prototype.addMethod = function (mid, cid, ins, outs) {
                _super.prototype.addMethod.call(this, mid, cid, ins, outs);
                if (cid in this.cids) {
                    u.arraySet.add(this.cids[cid], mid);
                }
                else {
                    this.cids[cid] = [mid];
                }
            };
            /*----------------------------------------------------------------
             * Remove variable / update cache
             */
            CachingConstraintGraph.prototype.removeVariable = function (vid) {
                delete this.vids[vid];
                _super.prototype.removeVariable.call(this, vid);
            };
            /*----------------------------------------------------------------
             * Remove method / update cache
             */
            CachingConstraintGraph.prototype.removeMethod = function (mid) {
                var cid = this.graph.getLabel(mid);
                var allmids = this.cids[cid];
                u.arraySet.remove(allmids, mid);
                if (allmids.length == 0) {
                    delete this.cids[cid];
                }
                _super.prototype.removeMethod.call(this, mid);
            };
            /*----------------------------------------------------------------
             * Get variables from cache
             */
            CachingConstraintGraph.prototype.variables = function () {
                return Object.keys(this.vids);
            };
            /*----------------------------------------------------------------
             * Get constraints from cache
             */
            CachingConstraintGraph.prototype.constraints = function () {
                return Object.keys(this.cids);
            };
            /*----------------------------------------------------------------
             * Look up method from cache
             */
            CachingConstraintGraph.prototype.methodsForConstraint = function (cid) {
                var mids = this.cids[cid];
                return mids ? u.arraySet.clone(mids) : [];
            };
            return CachingConstraintGraph;
        }(NoCachingConstraintGraph));
        graph.CachingConstraintGraph = CachingConstraintGraph;
    })(graph = hd.graph || (hd.graph = {}));
})(hd || (hd = {}));
/*####################################################################
 * The SolutionGraph interface and class.
 *
 * A SolutionGraph is just a constraint graph; however, it is defined
 * as a subgraph of another constraint graph.  In hypergraph
 * terminology, the solution graph contains all the nodes (variables)
 * of the original graph, and some but not all the hyperedges
 * (methods).  More specifically, the solution graph contains at most
 * one method from each constraint of the original constraint graph.
 *
 * To help enforce this, the solution graph keeps a pointer to the
 * constraint graph it is a subset of.  Rather than supporting the
 * arbitrary editing functions of ConstraintGraph, the SolutionGraph
 * only supports adding and removing methods of the original
 * constraint graph.
 */
var hd;
(function (hd) {
    var graph;
    (function (graph) {
        /*==================================================================
         * A solution graph with light caching.
         */
        var CachingSolutionGraph = /** @class */ (function (_super) {
            __extends(CachingSolutionGraph, _super);
            function CachingSolutionGraph() {
                return _super !== null && _super.apply(this, arguments) || this;
            }
            /*----------------------------------------------------------------
             * Remove any hyperedges for specified constraint; then add
             * specified hyeredge.
             *
             * Note that it's fine if there are no hyperedges for the
             * specified constraint.  Similarly, "mid" may be "null" to
             * indicate that there should be no hyperedges for the specified
             * constraint.
             */
            CachingSolutionGraph.prototype.selectMethod = function (cid, mid, inputs, outputs) {
                if (!mid) {
                    mid = null;
                }
                var oldmid = this.selectedForConstraint(cid);
                if (oldmid != mid) {
                    if (oldmid) {
                        this.removeMethod(oldmid);
                    }
                    if (mid) {
                        this.addMethod(mid, cid, inputs, outputs);
                    }
                }
            };
            /*----------------------------------------------------------------
             * Query which method for specified constraint is currently in
             * the solution graph.
             */
            CachingSolutionGraph.prototype.selectedForConstraint = function (cid) {
                var methods = this.methodsForConstraint(cid);
                var method;
                if (methods) {
                    method = methods[0];
                }
                return method ? method : null;
            };
            return CachingSolutionGraph;
        }(graph.CachingConstraintGraph));
        graph.CachingSolutionGraph = CachingSolutionGraph;
    })(graph = hd.graph || (hd.graph = {}));
})(hd || (hd = {}));
/*====================================================================
 * There are no stay constraints in the model; they exist only in the
 * constraint graph.  Thus, we create ids for them, but the ids do not
 * map to any actual objects.  The mapping between a variable id and
 * the corresponding stay constraint id is performed by a simple
 * string manipulation: simply take the id of the variable and append
 * "#sc" to the end.  Similarly, the single method of the stay
 * constraint (the "stay method") simply takes the id of the variable
 * and appends "#sm" to the end.  The following functions facilitate
 * that mapping.
 */
var hd;
(function (hd) {
    var graph;
    (function (graph) {
        /*------------------------------------------------------------------
         * Map variable id to stay method id.
         */
        function stayMethod(vid) {
            return vid + '#sm';
        }
        graph.stayMethod = stayMethod;
        /*------------------------------------------------------------------
         * Map variable id to stay constraint id.
         */
        function stayConstraint(vid) {
            return vid + '#sc';
        }
        graph.stayConstraint = stayConstraint;
        /*------------------------------------------------------------------
         * Test whether specified method id is a stay method id.
         */
        function isStayMethod(mid) {
            return (mid.substr(-3) == '#sm');
        }
        graph.isStayMethod = isStayMethod;
        /*------------------------------------------------------------------
         * Test whether specified method id is NOT a stay method id.
         */
        function isNotStayMethod(cid) {
            return (cid.substr(-3) != '#sm');
        }
        graph.isNotStayMethod = isNotStayMethod;
        /*------------------------------------------------------------------
         * Test whether specified constraint id is a stay constraint id.
         */
        function isStayConstraint(cid) {
            return (cid.substr(-3) == '#sc');
        }
        graph.isStayConstraint = isStayConstraint;
        /*------------------------------------------------------------------
         * Test whether specified constraint id is NOT a stay constraint id.
         */
        function isNotStayConstraint(cid) {
            return (cid.substr(-3) != '#sc');
        }
        graph.isNotStayConstraint = isNotStayConstraint;
    })(graph = hd.graph || (hd.graph = {}));
})(hd || (hd = {}));
//
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (b.hasOwnProperty(p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
var hd;
(function (hd) {
    var model;
    (function (model) {
        model.debugIds = true;
        var count = 0;
        function makeId(name) {
            var pieces = [];
            if (model.debugIds) {
                pieces.push(name);
            }
            pieces.push('#');
            pieces.push(++count);
            return pieces.join('');
        }
        model.makeId = makeId;
    })(model = hd.model || (hd.model = {}));
})(hd || (hd = {}));
/*####################################################################
 * The Variable class
 */
var hd;
(function (hd) {
    var model;
    (function (model) {
        var u = hd.utility;
        var r = hd.reactive;
        /*==================================================================
         * Variable in the property model.
         */
        var Variable = /** @class */ (function () {
            /*----------------------------------------------------------------
             * Initialize members.  Optional EqualityPredicate is used to
             * determine when value has changed.
             */
            function Variable(name, value, eq) {
                // Is the stay constraint created with max or min priority?
                this.optional = 2 /* Min */;
                // Error associated with this variable
                this.error = new r.ScheduledSignal(null);
                // Is value stale?
                this.stale = new r.ScheduledSignal(false, u.doubleEqual);
                // Is the variable a source?
                this.source = new r.ScheduledSignal(false, u.doubleEqual);
                // Is the variable pending?
                this.pending = new r.ScheduledSignal(false, u.doubleEqual);
                // Is the variable contributing to an output?
                this.contributing = new r.ScheduledSignal(u.Fuzzy.Yes, u.doubleEqual);
                // Could the variable contribute to an output if edited?
                this.relevant = new r.ScheduledSignal(u.Fuzzy.Yes, u.doubleEqual);
                // Publishes events when value is touched or changed
                this.changes = new r.BasicObservable();
                // Promise waiting to be assigned
                this.staged = null;
                this.id = model.makeId(name);
                this.name = name;
                this.value = new r.ScheduledSignal(undefined, eq);
                this.ladder = new r.PromiseLadder();
                // connect ladder to value
                this.ladder.addObserver(this, this.onLadderNext, this.onLadderError, null);
                if (value !== undefined) {
                    var p;
                    if (value instanceof r.Promise) {
                        p = value;
                    }
                    else {
                        p = new r.Promise();
                        if (r.plogger) {
                            r.plogger.register(p, this.name, "variable initialization");
                        }
                        p.resolve(value);
                    }
                    this.makePromise(p);
                }
                this.setCommand = new model.Command("set " + name, undefined, [], [], [this]);
            }
            /*----------------------------------------------------------------
             * Human readable name
             */
            Variable.prototype.toString = function () {
                return this.name;
            };
            /*----------------------------------------------------------------
             * Make a promise to set the variable's value later.
             *
             * Broken into two stages:  make the promise, then commit the
             * promise.
             */
            Variable.prototype.makePromise = function (promise) {
                if (!(promise instanceof r.Promise)) {
                    var value = promise;
                    promise = new r.Promise();
                    if (r.plogger) {
                        r.plogger.register(promise, this.name, 'variable update');
                    }
                    promise.resolve(value);
                }
                this.staged = promise;
            };
            Variable.prototype.commitPromise = function () {
                if (this.staged) {
                    this.ladder.addPromise(this.staged);
                    this.staged = null;
                    if (this.pending.get() == false) {
                        this.pending.set(true);
                        this.stale.set(false);
                        this.changes.sendNext({ type: 2 /* pending */, vv: this });
                    }
                }
            };
            /*----------------------------------------------------------------
             * Get pending promise if there is one; current promise otherwise.
             */
            Variable.prototype.getStagedPromise = function () {
                return this.staged || this.ladder.getCurrentPromise();
            };
            /*----------------------------------------------------------------
             * Get a promise for the variable's value (ignoring pending).
             */
            Variable.prototype.getCurrentPromise = function () {
                return this.ladder.getCurrentPromise();
            };
            /*----------------------------------------------------------------
             * Get a promise to be forwarded with the current variable value
             * (ignoring pending).
             */
            Variable.prototype.getForwardedPromise = function () {
                var p = new r.Promise();
                if (r.plogger) {
                    r.plogger.register(p, this.name + '#fwd', ' forwarded');
                }
                this.ladder.forwardPromise(p);
                return p;
            };
            /*----------------------------------------------------------------
             */
            Variable.prototype.get = function () {
                return this.value.get();
            };
            /*----------------------------------------------------------------
             */
            Variable.prototype.set = function (value) {
                this.makePromise(value);
                this.changes.sendNext({ type: 0 /* changed */, vv: this });
            };
            /*----------------------------------------------------------------
             */
            Variable.prototype.commandSet = function (value) {
                var cmd = Object.create(this.setCommand);
                cmd.result = value;
                this.changes.sendNext({ type: 4 /* command */, vv: this, cmd: cmd });
            };
            /*----------------------------------------------------------------
             * Observer: subscribing to a variable == subscribing to its value
             */
            Variable.prototype.addObserver = function () {
                return this.value.addObserver.apply(this.value, arguments);
            };
            /*----------------------------------------------------------------
             * Observer: subscribing to a variable == subscribing to its value
             */
            Variable.prototype.removeObserver = function (observer) {
                return this.value.removeObserver(observer);
            };
            /*----------------------------------------------------------------
             * Observable: widget produces an error
             */
            Variable.prototype.onError = function (error) {
                var p = new r.Promise();
                if (r.plogger) {
                    r.plogger.register(p, this.name, 'variable update');
                }
                p.reject(error);
                this.makePromise(p);
                this.changes.sendNext({ type: 0 /* changed */, vv: this });
            };
            /*----------------------------------------------------------------
             * Observable: unused - don't care if widget completes
             */
            Variable.prototype.onCompleted = function () {
                // No-op
            };
            /*----------------------------------------------------------------
             * Ladder produced a good value.
             */
            Variable.prototype.onLadderNext = function (value) {
                this.value.set(value);
                this.error.set(null);
                this.stale.set(this.ladder.currentFailed());
                if (this.ladder.isSettled()) {
                    this.pending.set(false);
                    this.changes.sendNext({ type: 3 /* settled */, vv: this });
                }
            };
            /*----------------------------------------------------------------
             * Ladder produced an error.
             */
            Variable.prototype.onLadderError = function (error) {
                if (error !== null) {
                    this.error.set(error);
                }
                this.stale.set(this.ladder.currentFailed());
                if (this.ladder.isSettled()) {
                    this.pending.set(false);
                    this.changes.sendNext({ type: 3 /* settled */, vv: this });
                }
            };
            return Variable;
        }());
        model.Variable = Variable;
        Variable.prototype.onNext = Variable.prototype.commandSet;
    })(model = hd.model || (hd.model = {}));
})(hd || (hd = {}));
/*####################################################################
 * The Method class.
 */
var hd;
(function (hd) {
    var config;
    (function (config) {
        config.forwardPriorGens = false;
    })(config = hd.config || (hd.config = {}));
})(hd || (hd = {}));
(function (hd) {
    var model;
    (function (model) {
        /*==================================================================
         *  A method is an operation that is part of a constraint.  As such,
         *  it keeps track of input and output variables as they appear in
         *  the constraint graph.  Every input should be in inputVars, every
         *  output should be in outputVars.
         */
        var Method = /** @class */ (function () {
            /*----------------------------------------------------------------
             * Initialize members
             */
            function Method(name, fn, inputs, priorFlags, outputs, inputVars, outputVars) {
                // Is this an external operation?  (Does it trigger an update after execution?)
                this.external = false;
                this.id = model.makeId(name);
                this.name = name;
                if (typeof fn === 'function') {
                    this.fn = fn;
                }
                else {
                    this.result = fn;
                }
                this.inputs = inputs;
                this.priorFlags = priorFlags;
                this.outputs = outputs;
                this.inputVars = inputVars;
                this.outputVars = outputVars;
            }
            /*----------------------------------------------------------------
             * Human readable name
             */
            Method.prototype.toString = function () {
                return this.name;
            };
            return Method;
        }());
        model.Method = Method;
    })(model = hd.model || (hd.model = {}));
})(hd || (hd = {}));
/*####################################################################
 * The Constraint class.
 */
var hd;
(function (hd) {
    var model;
    (function (model) {
        ;
        /*==================================================================
         * A constraint in the property model.
         */
        var Constraint = /** @class */ (function () {
            /*----------------------------------------------------------------
             * Initialize members
             */
            function Constraint(name, variables, touchVariables) {
                // Variables used by this constraint
                this.variables = [];
                // Methods in this constraint
                this.methods = [];
                // Is this constraint optional, and, if so, should it be added
                //   with max or min priority?
                this.optional = 0 /* Default */;
                this.id = model.makeId(name);
                this.name = name;
                this.variables = variables;
                if (touchVariables && touchVariables.length) {
                    this.touchVariables = touchVariables;
                }
            }
            /*----------------------------------------------------------------
             * Human readable name
             */
            Constraint.prototype.toString = function () {
                return this.name;
            };
            /*----------------------------------------------------------------
             * Add new method to constraint
             */
            Constraint.prototype.addMethod = function (method) {
                this.methods.push(method);
            };
            // Static constants for min/max strength
            Constraint.WeakestStrength = Number.MIN_VALUE;
            Constraint.RequiredStrength = Number.MAX_VALUE;
            return Constraint;
        }());
        model.Constraint = Constraint;
    })(model = hd.model || (hd.model = {}));
})(hd || (hd = {}));
/*####################################################################
 * Classes/interfaces related to components.
 */
var hd;
(function (hd) {
    var model;
    (function (model) {
        var u = hd.utility;
        var r = hd.reactive;
        /*******************************************************************
           Helpers
         ******************************************************************/
        // getter for constant
        function pathConstant(p) {
            return p.isConstant();
        }
        // lexicographical comparison for array
        function compareAnys(a, b) {
            for (var i = 0, l = a.length, m = b.length; i < l && i < m; ++i) {
                if (a[i] !== b[i]) {
                    var ai = a[i];
                    var bi = b[i];
                    var cmp;
                    if ((ai instanceof model.Variable || ai instanceof model.Constraint || ai instanceof model.Command) &&
                        (bi instanceof model.Variable || bi instanceof model.Constraint || bi instanceof model.Command)) {
                        cmp = ai.id.localeCompare(bi.id);
                    }
                    else if (Array.isArray(ai) && Array.isArray(bi)) {
                        cmp = compareAnys(ai, bi);
                    }
                    else if (typeof ai === 'number' && typeof bi === 'number') {
                        cmp = ai - bi;
                    }
                    else if (typeof ai === 'string' && typeof bi === 'string') {
                        cmp = ai.localeCompare(bi);
                    }
                    else {
                        cmp = (ai + "").localeCompare(bi + "");
                    }
                    if (cmp != 0) {
                        return cmp;
                    }
                }
            }
            return l - m;
        }
        // find the differences between two SORTED arrays
        function listDiff(a, b, compare) {
            var leftOnly = [];
            var leftShared = [];
            var rightOnly = [];
            var rightShared = [];
            var i = 0;
            var j = 0;
            var l = a.length;
            var m = b.length;
            while (i < l && j < m) {
                var cmp = compare(a[i], b[j]);
                if (cmp < 0) {
                    leftOnly.push(a[i]);
                    ++i;
                }
                else if (cmp > 0) {
                    rightOnly.push(b[j]);
                    ++j;
                }
                else {
                    leftShared.push(a[i]);
                    rightShared.push(b[j]);
                    ++i;
                    ++j;
                }
            }
            while (i < l) {
                leftOnly.push(a[i]);
                ++i;
            }
            while (j < m) {
                rightOnly.push(b[j]);
                ++j;
            }
            return { leftOnly: leftOnly, leftShared: leftShared,
                rightOnly: rightOnly, rightShared: rightShared };
        }
        function addToList(list, value, interpolations) {
            if (interpolations == 0) {
                list.push(value);
            }
            else if (interpolations == 1) {
                list.push.apply(list, value);
            }
            else {
                for (var i = 0, l = value.length; i < l; ++i) {
                    addToList(list, value[i], interpolations - 1);
                }
            }
        }
        // Wrapper class to represent touch dependency
        var TouchDep = /** @class */ (function () {
            function TouchDep(from, to) {
                this.from = from;
                this.to = to;
            }
            return TouchDep;
        }());
        model.TouchDep = TouchDep;
        // Wrapper class to represent output
        var Output = /** @class */ (function () {
            function Output(variable) {
                this.variable = variable;
            }
            return Output;
        }());
        model.Output = Output;
        // getter for element associated with instance
        function getElement(inst) {
            return inst.element;
        }
        /*==================================================================
         * Abstract base class for all templates.
         * For generality, this class does not interact directly with paths.
         * That means its the derived class' responsibility to:
         * - in constructor, subscribe to any paths used by the template
         * - in constructor, pick a path and assign to master
         * - in destructor, unsubscribe to all paths
         */
        var Template = /** @class */ (function (_super) {
            __extends(Template, _super);
            function Template() {
                var _this = _super !== null && _super.apply(this, arguments) || this;
                // All paths used by this template
                _this.paths = [];
                // All instances created from this template, in sorted order
                _this.instances = [];
                // Have their been any changes since the last update?
                _this.changed = true;
                return _this;
            }
            /*----------------------------------------------------------------
             */
            Template.prototype.addPath = function (path) {
                if (u.arraySet.add(this.paths, path)) {
                    path.addObserver(this);
                }
            };
            Template.prototype.addPaths = function (paths) {
                paths.forEach(this.addPath, this);
            };
            /*----------------------------------------------------------------
             */
            Template.prototype.isConstant = function () {
                return this.paths.every(pathConstant);
            };
            /*----------------------------------------------------------------
             * To be implemented by derived classes
             */
            // Abstract:  Create an instance for a given position
            Template.prototype.define = function (pos) {
                throw "Attempt to call abstract method";
            };
            // Abstract:  Order two instances
            Template.prototype.compare = function (a, b) {
                throw "Attempt to call abstract method";
            };
            // Abstract:  Create element for instance
            Template.prototype.create = function (inst) {
                throw "Attempt to call abstract method";
            };
            Template.prototype.addObserver = function (object, onNext, onError, onCompleted, id) {
                var added;
                if (arguments.length == 1) {
                    added = _super.prototype.addObserver.call(this, object);
                }
                else {
                    added = _super.prototype.addObserver.call(this, object, onNext, onError, onCompleted, id);
                }
                if (added && this.changed) {
                    added.onNext(this);
                }
                return added;
            };
            /*----------------------------------------------------------------
             * Recalculate all instances and report any changes.
             */
            Template.prototype.update = function (changes) {
                var newInstances = [];
                if (this.paths.length > 0) {
                    for (var itr = new model.PathSetIterator(this.paths); itr.pos !== null; itr.next()) {
                        var inst = this.define(itr.pos);
                        if (inst) {
                            newInstances.push(inst);
                        }
                    }
                }
                else {
                    var inst = this.define({});
                    if (inst) {
                        newInstances.push(inst);
                    }
                }
                // Sort the list
                newInstances.sort(this.compare);
                // Calculate difference between this list and our new list
                var diff = listDiff(this.instances, newInstances, this.compare);
                // Put leftOnly on the remove list
                if (diff.leftOnly.length > 0) {
                    changes.removes.push.apply(changes.removes, diff.leftOnly.map(getElement));
                }
                // Copy over elements for shared
                for (var i = 0, l = diff.leftShared.length; i < l; ++i) {
                    diff.rightShared[i].element = diff.leftShared[i].element;
                }
                // Create elements for rightOnly and put on the add list
                if (diff.rightOnly.length > 0) {
                    diff.rightOnly.forEach(function (inst) {
                        this.create(inst);
                        changes.adds.push(inst.element);
                    }, this);
                }
                // Record results
                this.instances = newInstances;
                this.changed = false;
            };
            /*----------------------------------------------------------------
             * All current elements created by this template
             */
            Template.prototype.getElements = function () {
                return this.instances.map(getElement);
            };
            /*----------------------------------------------------------------
             * When a path used by this template changes.  Currently, we
             * ignore the position that changes and just always recalculate
             * all positions.
             */
            Template.prototype.onNext = function () {
                if (!this.changed) {
                    this.changed = true;
                    this.sendNext(this);
                }
            };
            Template.prototype.onError = function () { };
            Template.prototype.onCompleted = function () { };
            /*----------------------------------------------------------------
             */
            Template.prototype.destruct = function () {
                for (var i = 0, l = this.paths.length; i < l; ++i) {
                    this.paths[i].removeObserver(this);
                }
            };
            return Template;
        }(r.BasicObservable));
        var MethodTemplate = /** @class */ (function () {
            /*----------------------------------------------------------------
             * Look up paths, but don't subscribe; the constraint template
             * handles that
             */
            function MethodTemplate(mspec, lookup) {
                this.inputs = u.multiArray.map(mspec.inputs, lookup.get, lookup);
                if (mspec.priorFlags) {
                    this.priorFlags = u.multiArray.copy(mspec.priorFlags);
                }
                this.outputs = u.multiArray.map(mspec.outputs, lookup.get, lookup);
                this.fn = mspec.fn;
                this.name = u.multiArray.toString(mspec.inputs) + '->' +
                    u.multiArray.toString(mspec.outputs);
            }
            /*----------------------------------------------------------------
             * Create an instance for a given position
             */
            MethodTemplate.prototype.define = function (pos) {
                var get = function (p) { return p.get(pos); };
                var ins = u.multiArray.map(this.inputs, get);
                var outs = u.multiArray.map(this.outputs, get);
                // Check outputs:  all variables, no duplicates
                var outVars = u.multiArray.flatten(outs);
                for (var i = 0, l = outVars.length; i < l; ++i) {
                    if (outVars[i] === undefined) {
                        return null;
                    }
                    if (!(outVars[i] instanceof model.Variable)) {
                        console.warn('Method cannot be instantiated with non-variable output: ' + this.name);
                        return null;
                    }
                    for (var j = i + 1; j < l; ++j) {
                        if (outVars[i] === outVars[j]) {
                            console.warn('Method cannot be instantiated with duplicate output: ' + this.name);
                            return null;
                        }
                    }
                }
                // Recursively check inputs:  current-value variable cannot be output
                var checkInputs = function (ins, priors) {
                    for (var i = 0, l = ins.length; i < l; ++i) {
                        if (ins[i] === undefined) {
                            return false;
                        }
                        else if (Array.isArray(ins[i])) {
                            if (!checkInputs(ins[i], priors ? priors[i] : undefined)) {
                                return false;
                            }
                        }
                        else if (ins[i] instanceof model.Variable &&
                            !(priors && priors[i]) &&
                            outVars.indexOf(ins[i]) >= 0) {
                            console.warn('Method cannot be instantiated with same variable as input and output: ' +
                                this.name);
                            return false;
                        }
                    }
                    return true;
                };
                if (!checkInputs(ins, this.priorFlags)) {
                    return null;
                }
                return { inputs: ins, outputs: outs };
            };
            /*----------------------------------------------------------------
             * Create element for instance; requires list of other variables
             * in the constraint.
             */
            MethodTemplate.prototype.create = function (inst, vars) {
                var outVars = u.multiArray.flatten(inst.outputs);
                return new model.Method(this.name, this.fn, inst.inputs, this.priorFlags, inst.outputs, u.arraySet.difference(vars, outVars), outVars);
            };
            return MethodTemplate;
        }());
        /*==================================================================
         * Constraint Template.
         */
        var ConstraintInstance = /** @class */ (function () {
            function ConstraintInstance(all, variables, methods, touchVariables, pos) {
                this.all = all;
                this.variables = variables;
                this.methods = methods;
                this.touchVariables = touchVariables;
                this.pos = pos;
            }
            return ConstraintInstance;
        }());
        var ConstraintTemplate = /** @class */ (function (_super) {
            __extends(ConstraintTemplate, _super);
            /*----------------------------------------------------------------
             * Initialize, look up paths, subscribe
             */
            function ConstraintTemplate(spec, lookup) {
                var _this = _super.call(this) || this;
                _this.addPaths(_this.variables = spec.variables.map(lookup.get, lookup));
                _this.methods = spec.methods.map(function (mspec) {
                    return new MethodTemplate(mspec, lookup);
                });
                _this.optional = spec.optional;
                if (spec.touchVariables && spec.touchVariables.length) {
                    _this.addPaths(_this.touchVariables = spec.touchVariables.map(lookup.get, lookup));
                }
                _this.name = spec.variables.join(',');
                return _this;
            }
            /*----------------------------------------------------------------
             * Create an instance for a given position.
             */
            ConstraintTemplate.prototype.define = function (pos) {
                var all = [];
                for (var i = 0, l = this.variables.length; i < l; ++i) {
                    var vv = this.variables[i].get(pos);
                    if (vv === undefined) {
                        return null;
                    }
                    addToList(all, vv, this.variables[i].slices);
                }
                var vvs = all.filter(u.isType(model.Variable));
                var minsts = [];
                var hasMethods = false;
                for (var i = 0, l = this.methods.length; i < l; ++i) {
                    var minst = this.methods[i].define(pos);
                    if (minst) {
                        minsts[i] = minst;
                        hasMethods = true;
                    }
                }
                if (hasMethods) {
                    var tvs;
                    if (this.touchVariables && this.touchVariables.length) {
                        tvs = [];
                        for (var i = 0, l = this.touchVariables.length; i < l; ++i) {
                            var vv = this.touchVariables[i].get(pos);
                            if (vv instanceof model.Variable) {
                                u.arraySet.add(tvs, vv);
                            }
                        }
                    }
                    return new ConstraintInstance(all, vvs, minsts, tvs, pos);
                }
                else {
                    return null;
                }
            };
            /*----------------------------------------------------------------
             * Order two instances
             */
            ConstraintTemplate.prototype.compare = function (a, b) {
                var cmp = compareAnys(a.all, b.all);
                if (cmp == 0 && a.touchVariables) {
                    cmp = compareAnys(a.touchVariables, b.touchVariables);
                }
                return cmp;
            };
            /*----------------------------------------------------------------
             * Create element for instance
             */
            ConstraintTemplate.prototype.create = function (inst) {
                var name = this.name.replace(/\[(\d*)([a-zA-Z][\w$]*)/g, function (all, n, v) {
                    return '[' + n + inst.pos[v];
                });
                var cc = new model.Constraint(name, inst.variables, inst.touchVariables);
                cc.optional = this.optional;
                for (var i = 0, l = inst.methods.length; i < l; ++i) {
                    if (inst.methods[i]) {
                        cc.addMethod(this.methods[i].create(inst.methods[i], inst.variables));
                    }
                }
                inst.element = cc;
            };
            return ConstraintTemplate;
        }(Template));
        /*==================================================================
         * Command Template.
         */
        var CommandInstance = /** @class */ (function () {
            function CommandInstance(inputs, outputs) {
                this.inputs = inputs;
                this.outputs = outputs;
            }
            return CommandInstance;
        }());
        var CommandTemplate = /** @class */ (function (_super) {
            __extends(CommandTemplate, _super);
            /*----------------------------------------------------------------
             * Initialize, look up paths, subscribe
             */
            function CommandTemplate(cmdspec, lookup) {
                var _this = _super.call(this) || this;
                _this.inputs = u.multiArray.map(cmdspec.inputs, lookup.get, lookup);
                var inputPaths = u.multiArray.flatten(_this.inputs);
                _this.addPaths(inputPaths);
                if (cmdspec.priorFlags) {
                    _this.priorFlags = u.multiArray.copy(cmdspec.priorFlags);
                }
                _this.outputs = u.multiArray.map(cmdspec.outputs, lookup.get, lookup);
                var outputPaths = u.multiArray.flatten(_this.outputs);
                _this.addPaths(outputPaths);
                _this.fn = cmdspec.fn;
                _this.synchronous = cmdspec.synchronous;
                _this.name = u.multiArray.toString(cmdspec.inputs) + '->' +
                    u.multiArray.toString(cmdspec.outputs);
                _this.activate = _this.activate.bind(_this);
                return _this;
            }
            /*----------------------------------------------------------------
             * Create an instance for a given position.
             */
            CommandTemplate.prototype.define = function (pos) {
                var get = function (p) { return p.get(pos); };
                var ins = u.multiArray.map(this.inputs, get);
                var outs = u.multiArray.map(this.outputs, get);
                // Check outputs:  all variables, no duplicates
                var outVars = u.multiArray.flatten(outs);
                for (var i = 0, l = outVars.length; i < l; ++i) {
                    if (outVars[i] === undefined) {
                        return null;
                    }
                    if (!(outVars[i] instanceof model.Variable)) {
                        console.warn('Command cannot be instantiated with non-variable output: ' + this.name);
                        return null;
                    }
                    for (var j = i + 1; j < l; ++j) {
                        if (outVars[i] === outVars[j]) {
                            console.warn('Command cannot be instantiated with duplicate output: ' + this.name);
                            return null;
                        }
                    }
                }
                // Recursively check inputs:  current-value variable cannot be output
                var checkInputs = function (ins, priors) {
                    for (var i = 0, l = ins.length; i < l; ++i) {
                        if (ins[i] === undefined) {
                            return false;
                        }
                        else if (Array.isArray(ins[i])) {
                            if (!checkInputs(ins[i], priors ? priors[i] : undefined)) {
                                return false;
                            }
                        }
                        else if (ins[i] instanceof model.Variable &&
                            !(priors && priors[i]) &&
                            outVars.indexOf(ins[i]) >= 0) {
                            console.warn('Command cannot be instantiated with same variable as input and output: ' +
                                this.name);
                            return false;
                        }
                    }
                    return true;
                };
                if (!checkInputs(ins, this.priorFlags)) {
                    return null;
                }
                return new CommandInstance(ins, outs);
            };
            /*----------------------------------------------------------------
             * Order two instances
             */
            CommandTemplate.prototype.compare = function (a, b) {
                var cmp = compareAnys(a.inputs, b.inputs);
                if (cmp == 0) {
                    cmp = compareAnys(a.outputs, b.outputs);
                }
                return cmp;
            };
            /*----------------------------------------------------------------
             * Create element for instance
             */
            CommandTemplate.prototype.create = function (inst) {
                if (this.synchronous) {
                    inst.element = new model.SynchronousCommand(this.name, this.fn, inst.inputs, this.priorFlags, inst.outputs);
                }
                else {
                    inst.element = new model.Command(this.name, this.fn, inst.inputs, this.priorFlags, inst.outputs);
                }
            };
            /*----------------------------------------------------------------
             */
            CommandTemplate.prototype.activate = function () {
                var cmds = this.getElements();
                if (cmds.length) {
                    cmds[0].activate();
                }
            };
            return CommandTemplate;
        }(Template));
        /*==================================================================
         * TouchDep Template.
         */
        var TouchDepInstance = /** @class */ (function () {
            function TouchDepInstance(from, to) {
                this.from = from;
                this.to = to;
            }
            return TouchDepInstance;
        }());
        var TouchDepTemplate = /** @class */ (function (_super) {
            __extends(TouchDepTemplate, _super);
            /*----------------------------------------------------------------
             * Initialize, look up paths, subscribe
             */
            function TouchDepTemplate(spec, lookup) {
                var _this = _super.call(this) || this;
                _this.addPath(_this.from = lookup.get(spec.from));
                _this.addPath(_this.to = lookup.get(spec.to));
                _this.name = _this.from + '=>' + _this.to;
                return _this;
            }
            /*----------------------------------------------------------------
             * Create an instance for a given position.
             */
            TouchDepTemplate.prototype.define = function (pos) {
                var cc1 = this.from.get(pos);
                var cc2 = this.to.get(pos);
                if (cc1 !== cc2 &&
                    (cc1 instanceof model.Variable || cc1 instanceof model.Constraint) &&
                    (cc2 instanceof model.Variable || cc2 instanceof model.Constraint)) {
                    return new TouchDepInstance(cc1, cc2);
                }
                else {
                    return null;
                }
            };
            /*----------------------------------------------------------------
             * Order two instances
             */
            TouchDepTemplate.prototype.compare = function (a, b) {
                var cmp = a.from.id.localeCompare(b.from.id);
                if (cmp == 0) {
                    cmp = a.to.id.localeCompare(b.to.id);
                }
                return cmp;
            };
            /*----------------------------------------------------------------
             * Create element for instance
             */
            TouchDepTemplate.prototype.create = function (inst) {
                inst.element = new TouchDep(inst.from, inst.to);
            };
            return TouchDepTemplate;
        }(Template));
        /*==================================================================
         * Output Template;
         */
        var OutputInstance = /** @class */ (function () {
            function OutputInstance(variable) {
                this.variable = variable;
            }
            return OutputInstance;
        }());
        var OutputTemplate = /** @class */ (function (_super) {
            __extends(OutputTemplate, _super);
            /*----------------------------------------------------------------
             * Initialize, look up paths, subscribe
             */
            function OutputTemplate(spec, lookup) {
                var _this = _super.call(this) || this;
                _this.addPath(_this.variable = lookup.get(spec.variable));
                _this.name = '@' + spec.variable;
                return _this;
            }
            /*----------------------------------------------------------------
             * Create an instance for a given position.
             */
            OutputTemplate.prototype.define = function (pos) {
                var vv = this.variable.get(pos);
                if (vv instanceof model.Variable) {
                    return new OutputInstance(vv);
                }
            };
            /*----------------------------------------------------------------
             * Order two instances
             */
            OutputTemplate.prototype.compare = function (a, b) {
                return a.variable.id.localeCompare(b.variable.id);
            };
            /*----------------------------------------------------------------
             * Create element for instance
             */
            OutputTemplate.prototype.create = function (inst) {
                inst.element = new Output(inst.variable);
            };
            return OutputTemplate;
        }(Template));
        /*******************************************************************
          The Component type
         ******************************************************************/
        /*==================================================================
         * The actual data members of a component are held in a separate
         * class.  If a dynamic element does not use any references then
         * it is simply instantiated and stored; if it does, then we store
         * a template for the element.
         */
        var ComponentData = /** @class */ (function (_super) {
            __extends(ComponentData, _super);
            // Init
            function ComponentData(cmp) {
                var _this = _super.call(this) || this;
                // The static elements
                _this.elements = [];
                // Templates for dynamic elements
                _this.templates = [];
                // Any static elements added since the last update
                _this.added = [];
                // Any static elements removed since the last update
                _this.removed = [];
                // Templates whose paths have changed since last update
                _this.outdated = [];
                // Have any changes been made since last update?
                _this.changed = false;
                // All paths used by this component (to promote sharing among templates)
                _this.paths = {};
                _this.component = cmp;
                return _this;
            }
            /*----------------------------------------------------------------
             * Ensure this sends only a single changed message between
             * updates.
             */
            ComponentData.prototype.reportChanged = function () {
                if (!this.changed) {
                    this.changed = true;
                    this.sendNext(this.component);
                }
            };
            /*----------------------------------------------------------------
             * Return all elements currently in the component.
             */
            ComponentData.prototype.getElements = function () {
                return this.elements.concat(u.concatmap(this.templates, function (tmpl) {
                    return tmpl.getElements();
                }));
            };
            /*----------------------------------------------------------------
             * Adds a static element to the component
             */
            ComponentData.prototype.addStatic = function (element) {
                var added = false;
                if (u.arraySet.remove(this.removed, element)) {
                    added = true;
                }
                if (!u.arraySet.contains(this.elements, element) &&
                    u.arraySet.add(this.added, element)) {
                    added = true;
                    this.reportChanged();
                }
                return added;
            };
            /*----------------------------------------------------------------
             * Removes a static element to the component
             */
            ComponentData.prototype.removeStatic = function (element) {
                var removed = false;
                if (u.arraySet.remove(this.added, element)) {
                    removed = true;
                }
                if (u.arraySet.contains(this.elements, element) &&
                    u.arraySet.add(this.removed, element)) {
                    removed = true;
                    this.reportChanged();
                }
                return removed;
            };
            /*----------------------------------------------------------------
             * Adds a template to the component
             */
            ComponentData.prototype.addTemplate = function (tmpl) {
                if (tmpl.isConstant()) {
                    var changes = { removes: [], adds: [] };
                    tmpl.update(changes);
                    if (changes.adds.length > 0) {
                        changes.adds.forEach(this.addStatic, this);
                    }
                    else {
                        console.warn("Could not instantiate constant template: " + tmpl.name);
                    }
                }
                else {
                    if (u.arraySet.add(this.templates, tmpl)) {
                        tmpl.addObserver(this);
                    }
                }
            };
            /*----------------------------------------------------------------
             */
            ComponentData.prototype.update = function () {
                var removed = this.removed;
                this.removed = [];
                if (removed.length > 0) {
                    this.elements = u.arraySet.difference(this.elements, removed);
                }
                var added = this.added;
                this.added = [];
                if (added.length > 0) {
                    Array.prototype.push.apply(this.elements, added);
                }
                var result = { adds: added, removes: removed };
                if (this.outdated.length > 0) {
                    for (var i = 0, l = this.outdated.length; i < l; ++i) {
                        this.outdated[i].update(result);
                    }
                    this.outdated = [];
                }
                this.changed = false;
                return result;
            };
            /*----------------------------------------------------------------
             * Implement Observable<Template>
             */
            ComponentData.prototype.onNext = function (tmpl) {
                this.outdated.push(tmpl);
                this.reportChanged();
            };
            ComponentData.prototype.onError = function () { };
            ComponentData.prototype.onCompleted = function () { };
            /*----------------------------------------------------------------
             * Implement PathLookup
             */
            ComponentData.prototype.get = function (name) {
                var path = this.paths[name];
                if (!path) {
                    path = this.paths[name] = new model.Path(this.component, name);
                }
                return path;
            };
            ComponentData.prototype.addObserver = function (object, onNext, onError, onCompleted, id) {
                var added;
                if (arguments.length == 1) {
                    added = _super.prototype.addObserver.call(this, object);
                }
                else {
                    added = _super.prototype.addObserver.call(this, object, onNext, onError, onCompleted, id);
                }
                if (added && this.changed) {
                    added.onNext(this.component);
                }
                return added;
            };
            return ComponentData;
        }(r.BasicObservable));
        /*==================================================================
         * The component class itself has a hidden ComponentData field; all
         *   other fields are user-defined component properties.
         * (Member functions of the component class are all static.)
         */
        var Component = /** @class */ (function () {
            /*----------------------------------------------------------------
             * Copies any fields found in init
             */
            function Component(spec, init) {
                if (spec) {
                    Component.construct(this, spec, init);
                }
            }
            /*----------------------------------------------------------------
             * A "constructor" - add everything from component spec to the
             * given component.
             */
            Component.construct = function (cmp, spec, init) {
                if (!(cmp instanceof Component)) {
                    throw "Attempting to construct component using specification failed:  object not a component!";
                }
                if (init) {
                    if (typeof init !== 'object') {
                        throw "Invalid initialization object passed to Component.construct: " + init;
                    }
                }
                else {
                    init = {};
                }
                for (var i = 0, l = spec.constants.length; i < l; ++i) {
                    var tspec = spec.constants[i];
                    if (!(tspec.loc in cmp)) {
                        Component.addConstant(cmp, spec.constants[i]);
                    }
                }
                var variables = spec.variables.filter(function (vspec) {
                    return !(vspec.loc in cmp);
                });
                // Initialized/min
                for (var i = variables.length - 1; i >= 0; --i) {
                    var vspec = variables[i];
                    if ((vspec.optional === 2 /* Min */ &&
                        (vspec.init !== undefined || init[vspec.loc] !== undefined)) ||
                        (vspec.optional === 0 /* Default */ &&
                            init[vspec.loc] === undefined &&
                            vspec.init !== undefined)) {
                        Component.addVariable(cmp, vspec, init[vspec.loc]);
                    }
                }
                // Uninitialized/min
                for (var i = variables.length - 1; i >= 0; --i) {
                    var vspec = variables[i];
                    if ((vspec.optional === 0 /* Default */ || vspec.optional === 2 /* Min */) &&
                        vspec.init === undefined &&
                        init[vspec.loc] === undefined) {
                        Component.addVariable(cmp, vspec, init[vspec.loc]);
                    }
                }
                // Uninitialized/max
                for (var i = 0, l = variables.length; i < l; ++i) {
                    var vspec = variables[i];
                    if (vspec.optional === 1 /* Max */ &&
                        vspec.init === undefined &&
                        init[vspec.loc] === undefined) {
                        Component.addVariable(cmp, vspec, init[vspec.loc]);
                    }
                }
                // Initialized/max
                for (var i = 0, l = variables.length; i < l; ++i) {
                    var vspec = variables[i];
                    if ((vspec.optional === 1 /* Max */ &&
                        (vspec.init !== undefined || init[vspec.loc] !== undefined)) ||
                        (vspec.optional === 0 /* Default */ &&
                            init[vspec.loc] !== undefined)) {
                        Component.addVariable(cmp, vspec, init[vspec.loc]);
                    }
                }
                for (var i = 0, l = spec.nesteds.length; i < l; ++i) {
                    var nspec = spec.nesteds[i];
                    if (!(nspec.loc in cmp)) {
                        Component.addNestedComponent(cmp, nspec, init[nspec.loc]);
                    }
                }
                for (var i = 0, l = spec.references.length; i < l; ++i) {
                    var rspec = spec.references[i];
                    if (!(rspec.loc in cmp)) {
                        Component.addReference(cmp, rspec, init[rspec.loc]);
                    }
                }
                for (var i = 0, l = spec.constraints.length; i < l; ++i) {
                    Component.addConstraint(cmp, spec.constraints[i]);
                }
                for (var i = 0, l = spec.commands.length; i < l; ++i) {
                    Component.addCommand(cmp, spec.commands[i]);
                }
                for (var i = 0, l = spec.touchDeps.length; i < l; ++i) {
                    Component.addTouchDep(cmp, spec.touchDeps[i]);
                }
                for (var i = 0, l = spec.outputs.length; i < l; ++i) {
                    Component.addOutput(cmp, spec.outputs[i]);
                }
                return cmp;
            };
            /*----------------------------------------------------------------
             */
            Component.addConstant = function (cmp, spec) {
                cmp[spec.loc] = spec.value;
            };
            /*----------------------------------------------------------------
             * Add variable
             */
            Component.addVariable = function (cmp, spec, init) {
                var hd_data = cmp['#hd_data'];
                var vv = new model.Variable(spec.loc, init === undefined ? spec.init : init, spec.eq);
                if (spec.optional !== 0 /* Default */) {
                    vv.optional = spec.optional;
                }
                else {
                    vv.optional = (init === undefined ? 2 /* Min */ : 1 /* Max */);
                }
                hd_data.addStatic(vv);
                cmp[spec.loc] = vv;
            };
            /*----------------------------------------------------------------
             * Add nested component
             */
            Component.addNestedComponent = function (cmp, spec, init) {
                var hd_data = cmp['#hd_data'];
                var nested;
                if (typeof spec.cmpType === 'function') {
                    nested = new spec.cmpType();
                }
                else {
                    nested = new Component();
                    if (spec.cmpType) {
                        Component.construct(nested, spec.cmpType);
                    }
                }
                hd_data.addStatic(nested);
                cmp[spec.loc] = nested;
            };
            /*----------------------------------------------------------------
             * Add dynamic reference
             */
            Component.addReference = function (cmp, spec, init) {
                var prop = new r.BasicSignal(init, spec.eq);
                Component.defineReferenceAccessors(cmp, spec.loc, prop);
            };
            Component.defineReferenceAccessors = function (cmp, loc, prop) {
                Object.defineProperty(cmp, '$' + loc, { configurable: true,
                    enumerable: false,
                    value: prop
                });
                Object.defineProperty(cmp, loc, { configurable: true,
                    enumerable: true,
                    get: prop.get.bind(prop),
                    set: prop.set.bind(prop)
                });
            };
            /*----------------------------------------------------------------
             * Add constraint
             */
            Component.addConstraint = function (cmp, spec) {
                var hd_data = cmp['#hd_data'];
                var tmpl = new ConstraintTemplate(spec, hd_data);
                hd_data.addTemplate(tmpl);
                if (spec.loc) {
                    cmp[spec.loc] = tmpl;
                }
            };
            /*----------------------------------------------------------------
             * Add command
             */
            Component.addCommand = function (cmp, spec) {
                var hd_data = cmp['#hd_data'];
                var tmpl = new CommandTemplate(spec, hd_data);
                hd_data.addTemplate(tmpl);
                if (spec.loc && tmpl.isConstant()) {
                    cmp[spec.loc] = tmpl.instances[0].element;
                }
            };
            /*----------------------------------------------------------------
             * Add touch dependency
             */
            Component.addTouchDep = function (cmp, spec) {
                var hd_data = cmp['#hd_data'];
                hd_data.addTemplate(new TouchDepTemplate(spec, hd_data));
            };
            /*----------------------------------------------------------------
             * Add output
             */
            Component.addOutput = function (cmp, spec) {
                var hd_data = cmp['#hd_data'];
                hd_data.addTemplate(new OutputTemplate(spec, hd_data));
            };
            /*----------------------------------------------------------------
             * Update all templates which need it, but also record the changes
             *   which were made.
             */
            Component.update = function (cmp) {
                var hd_data = cmp['#hd_data'];
                return hd_data.update();
            };
            /*----------------------------------------------------------------
             * Getter
             */
            Component.changes = function (cmp) {
                return cmp['#hd_data'];
            };
            /*----------------------------------------------------------------
             * Getter
             */
            Component.elements = function (cmp) {
                var hd_data = cmp['#hd_data'];
                return hd_data.getElements();
            };
            /*----------------------------------------------------------------
             */
            Component.claim = function (cmp, el) {
                var hd_data = cmp['#hd_data'];
                return hd_data.addStatic(el);
            };
            /*----------------------------------------------------------------
             */
            Component.release = function (cmp, el) {
                var hd_data = cmp['#hd_data'];
                return hd_data.removeStatic(el);
            };
            /*----------------------------------------------------------------
             */
            Component.destruct = function (cmp) {
                var hd_data = cmp['#hd_data'];
                for (var i = 0, l = hd_data.templates.length; i < l; ++i) {
                    hd_data.templates[i].destruct();
                }
            };
            return Component;
        }());
        model.Component = Component;
        /*------------------------------------------------------------------
         * Rather than require every component subclass to call the component
         * constructor, we define a getter that creates it the first time it
         * is accessed.
         */
        Object.defineProperty(Component.prototype, '#hd_data', {
            get: function makeData() {
                var data = new ComponentData(this);
                Object.defineProperty(this, '#hd_data', { configurable: true,
                    enumerable: false,
                    value: data
                });
                return data;
            }
        });
    })(model = hd.model || (hd.model = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var model;
    (function (model) {
        var eqn;
        (function (eqn) {
            /*==================================================================
             * A numeric literal.
             */
            var Number = /** @class */ (function () {
                function Number(text) {
                    this.text = text;
                }
                Number.prototype.print = function () { return this.text; };
                Number.prototype.hasVar = function () { return false; };
                Number.prototype.find = function (varname) { return null; };
                Number.prototype.extractDivisor = function (name) { return null; };
                return Number;
            }());
            eqn.Number = Number;
            /*==================================================================
             * A variable reference.
             */
            var Variable = /** @class */ (function () {
                function Variable(name) {
                    this.name = name;
                }
                Variable.prototype.print = function () { return this.name; };
                Variable.prototype.hasVar = function () { return true; };
                Variable.prototype.find = function (varname) {
                    return this.name == varname ? [] : null;
                };
                Variable.prototype.extractDivisor = function (name) { return null; };
                return Variable;
            }());
            eqn.Variable = Variable;
            /*==================================================================
             * A unary operation.
             */
            var UnaryOperation = /** @class */ (function () {
                function UnaryOperation(op, expr) {
                    this.op = op;
                    this.expr = expr;
                }
                UnaryOperation.prototype.print = function () { return '(' + this.op + this.expr.print() + ')'; };
                UnaryOperation.prototype.hasVar = function () { return this.expr.hasVar(); };
                UnaryOperation.prototype.find = function (varname) {
                    var path = this.expr.find(varname);
                    if (path) {
                        path.unshift('expr');
                    }
                    ;
                    return path;
                };
                UnaryOperation.prototype.extractDivisor = function (name) {
                    return this.expr.extractDivisor(name);
                };
                return UnaryOperation;
            }());
            eqn.UnaryOperation = UnaryOperation;
            /*==================================================================
             * A binary operation.
             */
            var BinaryOperation = /** @class */ (function () {
                function BinaryOperation(left, op, right) {
                    this.left = left;
                    this.op = op;
                    this.right = right;
                    // Only extract divisor once
                    this.extracted = false;
                }
                BinaryOperation.prototype.print = function () {
                    return '(' + this.left.print() + this.op + this.right.print() + ')';
                };
                BinaryOperation.prototype.hasVar = function () { return this.left.hasVar() || this.right.hasVar(); };
                BinaryOperation.prototype.find = function (varname) {
                    var path = this.left.find(varname);
                    if (path) {
                        path.unshift('left');
                    }
                    else {
                        path = this.right.find(varname);
                        if (path) {
                            path.unshift('right');
                        }
                    }
                    return path;
                };
                BinaryOperation.prototype.extractDivisor = function (name) {
                    var div = this.right.extractDivisor(name);
                    if (!div) {
                        div = this.left.extractDivisor(name);
                        if (!div && this.op == '/') {
                            if (!this.extracted) {
                                if (this.right.hasVar()) {
                                    div = this.right;
                                    if (!(this.right instanceof Variable)) {
                                        this.right = new Variable(name);
                                    }
                                }
                                this.extracted = true;
                            }
                        }
                    }
                    return div;
                };
                return BinaryOperation;
            }());
            eqn.BinaryOperation = BinaryOperation;
            /*==================================================================
             * Record-type for equation.
             */
            var Equation = /** @class */ (function () {
                function Equation(left, op, right, vars) {
                    this.left = left;
                    this.op = op;
                    this.right = right;
                    this.vars = vars;
                }
                return Equation;
            }());
            eqn.Equation = Equation;
            /*==================================================================
             * Parse string to generate equation.
             */
            function parse(line) {
                var tokens = line.match(/((\d+(\.\d*)?)|(\.\d+))([eE][+-]?\d+)?|[\w\.$]+|[<>=]=|\S/g);
                var position = 0;
                var varcount = 0;
                var varaliases = {};
                var eq = parseEquation();
                if (position === tokens.length) {
                    return eq;
                }
                else {
                    throw "Unexpected token after equation ended: '" + tokens[position] + "'";
                }
                /*----------------------------------------------------------------
                 * Factor := number
                 *         | variable
                 *         | unop Expression
                 *         | ( Expression )
                 */
                function parseFactor() {
                    var tok = tokens[position++];
                    if (tok.match(/^\.?\d/)) {
                        return new Number(tok);
                    }
                    else if (tok.match(/^[\w\.$]+$/)) {
                        if (tok in varaliases) {
                            throw "Encountered duplicate variable: '" + tok + "'";
                        }
                        varaliases[tok] = 'v' + (++varcount);
                        return new Variable(varaliases[tok]);
                    }
                    else if (tok === '-') {
                        var expr = parseFactor();
                        return new UnaryOperation('-', expr);
                    }
                    else if (tok === '(') {
                        expr = parseExpression();
                        tok = tokens[position++];
                        if (tok !== ')') {
                            throw "Expected ')' but found '" + tok + "''";
                        }
                        return expr;
                    }
                    throw "Expected value but found '" + tok + "'";
                }
                /*----------------------------------------------------------------
                 * Term := Term * Term
                 *       | Term / Term
                 *       | Factor
                 */
                function parseTerm() {
                    var left = parseFactor();
                    var op = tokens[position];
                    while (op === '*' || op === '/') {
                        ++position;
                        var right = parseFactor();
                        left = new BinaryOperation(left, op, right);
                        op = tokens[position];
                    }
                    return left;
                }
                /*----------------------------------------------------------------
                 * Expression := Expression + Expression
                 *             | Expression - Expression
                 *             | Term
                 */
                function parseExpression() {
                    var left = parseTerm();
                    var op = tokens[position];
                    while (op === '+' || op === '-') {
                        ++position;
                        var right = parseTerm();
                        left = new BinaryOperation(left, op, right);
                        op = tokens[position];
                    }
                    return left;
                }
                /*----------------------------------------------------------------
                 * Equation := Expression == Expression
                 *           | Expression <= Expression
                 *           | Expression >= Expression
                 */
                function parseEquation() {
                    var left = parseExpression();
                    var op = tokens[position++];
                    if (!(op && op.match(/^[<>=]=$/))) {
                        throw "Expected equality but found '" + op + "'";
                    }
                    var right = parseExpression();
                    return new Equation(left, op, right, varaliases);
                }
            }
            eqn.parse = parse;
            /*==================================================================
             * Generate JS code for body of function which solves for given
             * variable.
             */
            function makeFunction(inputs, output, eq) {
                var toId = function (name) {
                    if (name in eq.vars) {
                        return eq.vars[name];
                    }
                    throw "Unknown variable '" + name + "'";
                };
                inputs = inputs.map(toId);
                output = toId(output);
                var expr = solve(eq, output);
                if (expr) {
                    var lines = [];
                    // Extract out each divisor to check for (== 0)
                    var i = 1;
                    var name;
                    do {
                        do {
                            name = 'd' + i++;
                        } while (name == output || inputs.indexOf(name) >= 0);
                        var d = expr.extractDivisor(name);
                        if (d) {
                            if (d instanceof Variable) {
                                lines.push('if (' + d.name + ' == 0) throw "Division by zero";');
                            }
                            else {
                                lines.push('var ' + name + ' = ' + d.print() + ';');
                                lines.push('if (' + name + ' == 0) throw "Division by zero";');
                            }
                        }
                    } while (d);
                    // Perform calculation
                    if (eq.op == '==') {
                        lines.push('return ' + expr.print() + ';');
                    }
                    else {
                        lines.push('if (' + eq.left.print() +
                            eq.op + eq.right.print() + ')');
                        lines.push('  return ' + output + ';');
                        lines.push('else');
                        lines.push('  return ' + expr.print() + ';');
                    }
                    var fnBody = lines.join('\n');
                    var args = [null].concat(inputs);
                    args.push(fnBody);
                    return new (Function.bind.apply(Function, args))();
                }
                else {
                    return null;
                }
            }
            eqn.makeFunction = makeFunction;
            /*==================================================================
             * Solve equation for one variable.
             */
            function solve(eq, name) {
                var path = eq.left.find(name);
                if (path) {
                    return reduce(eq.left, eq.right);
                }
                else {
                    path = eq.right.find(name);
                    if (path) {
                        return reduce(eq.right, eq.left);
                    }
                    else {
                        return null;
                    }
                }
                /*----------------------------------------------------------------
                 * Reduce left expression down until only variable remains;
                 * each reduction is mirrored in right expression.
                 */
                function reduce(left, right) {
                    if (path.length == 0) {
                        return right;
                    }
                    if (left instanceof UnaryOperation) {
                        var u = left;
                        path.shift();
                        return reduce(u.expr, new UnaryOperation(u.op, right));
                    }
                    else if (left instanceof BinaryOperation) {
                        var b = left;
                        if (b.op === '+') {
                            if (path.shift() === 'left') {
                                return reduce(b.left, new BinaryOperation(right, '-', b.right));
                            }
                            else {
                                return reduce(b.right, new BinaryOperation(right, '-', b.left));
                            }
                        }
                        else if (b.op === '-') {
                            if (path.shift() === 'left') {
                                return reduce(b.left, new BinaryOperation(right, '+', b.right));
                            }
                            else {
                                return reduce(b.right, new BinaryOperation(b.left, '-', right));
                            }
                        }
                        else if (b.op === '*') {
                            if (path.shift() === 'left') {
                                return reduce(b.left, new BinaryOperation(right, '/', b.right));
                            }
                            else {
                                return reduce(b.right, new BinaryOperation(right, '/', b.left));
                            }
                        }
                        else if (b.op === '/') {
                            if (path.shift() === 'left') {
                                return reduce(b.left, new BinaryOperation(right, '*', b.right));
                            }
                            else {
                                return reduce(b.right, new BinaryOperation(b.left, '/', right));
                            }
                        }
                    }
                }
            }
        })(eqn = model.eqn || (model.eqn = {}));
    })(model = hd.model || (hd.model = {}));
})(hd || (hd = {}));
/*####################################################################
 * The ComponentBuilder class.
 */
var hd;
(function (hd) {
    var model;
    (function (model) {
        var u = hd.utility;
        var r = hd.reactive;
        /*================================================================
         * The ComponentBuilder class represents our embedded-DSL for
         * creating components.
         *
         * The various factory methods spend a lot of time validating
         * parameters and massaging them to fit the parameters of the actual
         * object constructors.  (The object constructors themselves assume
         * all parameters have been validated and are in the expected
         * format.)
         */
        var ComponentBuilder = /** @class */ (function () {
            function ComponentBuilder() {
                // The spec we are building
                this.target = {
                    constants: [],
                    variables: [],
                    nesteds: [],
                    references: [],
                    constraints: [],
                    commands: [],
                    outputs: [],
                    touchDeps: []
                };
                // If the last thing created was a constraint or a method in a
                // constraint, this will point to the constraint; otherwise it
                // will be null
                this.lastConstraint = null;
                // If the last thing created was one or more variables, this will
                // point to the variables; otherwise it will be null
                this.lastVariables = null;
                this.usedLocs = {};
            }
            /*----------------------------------------------------------------
             * Get the spec constructed by this builder
             */
            ComponentBuilder.prototype.spec = function () {
                this.endAll();
                return this.target;
            };
            /*----------------------------------------------------------------
             * Get a component made according to the spec
             */
            ComponentBuilder.prototype.component = function (init) {
                this.endAll();
                return new model.Component(this.target, init);
            };
            /*----------------------------------------------------------------
             * Add a constant.
             */
            ComponentBuilder.prototype.constant = function (loc, value) {
                this.target.constants.push({ loc: loc, value: value });
                return this;
            };
            ComponentBuilder.prototype.variable = function (loc, init, eq) {
                this.endAll();
                if (this.invalidLoc(loc)) {
                    return this;
                }
                if (eq && typeof eq !== 'function') {
                    console.error("Variable equality predicate must be a function");
                    return this;
                }
                var vspec = { loc: loc, init: init, optional: 0 /* Default */, eq: eq };
                this.target.variables.push(vspec);
                this.usedLocs[loc] = true;
                this.lastVariables = vspec;
                return this;
            };
            ComponentBuilder.prototype.variables = function () {
                this.endAll();
                var varorder;
                var vardefs;
                var output;
                if (typeof arguments[0] === 'string') {
                    varorder = arguments[0].trim().split(/\s*,\s*/);
                    vardefs = arguments[1] || {};
                }
                else {
                    vardefs = arguments[0];
                    varorder = Object.keys(vardefs);
                }
                var vspecs = [];
                for (var i = 0, l = varorder.length; i < l; ++i) {
                    var loc = varorder[i];
                    this.variable(loc, vardefs[loc]);
                    if (this.lastVariables) {
                        vspecs.push(this.lastVariables);
                    }
                }
                this.lastVariables = vspecs.length > 0 ? vspecs : null;
                return this;
            };
            ComponentBuilder.prototype.nested = function (loc, cmpType) {
                this.endAll();
                if (this.invalidLoc(loc)) {
                    return this;
                }
                this.target.nesteds.push({ loc: loc, cmpType: cmpType });
                this.usedLocs[loc] = true;
                return this;
            };
            ComponentBuilder.prototype.reference = function (loc, eq) {
                this.endAll();
                if (this.invalidLoc(loc)) {
                    return this;
                }
                this.target.references.push({ loc: loc, eq: eq });
                this.usedLocs[loc] = true;
                return this;
            };
            /*----------------------------------------------------------------
             * Convenience method for many references at once
             */
            ComponentBuilder.prototype.references = function (locs) {
                locs.trim().split(/\s*,\s*/).forEach(function (loc) {
                    this.reference(loc);
                }, this);
                return this;
            };
            ComponentBuilder.prototype.constraint = function () {
                this.endAll();
                var loc, signature;
                if (arguments.length > 1) {
                    loc = arguments[0];
                    signature = arguments[1];
                }
                else {
                    signature = arguments[0];
                }
                var p = parseConstraint(signature);
                if (p == null) {
                    return this;
                }
                if (p.constraintVars.some(invalidPath) ||
                    (p.touchVars && p.touchVars.some(invalidPath))) {
                    return this;
                }
                this.lastConstraint = { variables: p.constraintVars,
                    methods: [],
                    optional: 0 /* Default */ };
                if (p.touchVars) {
                    this.lastConstraint.optional = 1 /* Max */;
                    this.lastConstraint.touchVariables = p.touchVars;
                }
                if (loc && !this.invalidLoc(loc)) {
                    this.lastConstraint.loc = loc;
                    this.usedLocs[this.lastConstraint.loc] = true;
                }
                return this;
            };
            // Complete the current constraint; no effect if no current constraint
            ComponentBuilder.prototype.endConstraint = function () {
                if (this.lastConstraint) {
                    this.target.constraints.push(this.lastConstraint);
                    this.lastConstraint = null;
                }
                return this;
            };
            ComponentBuilder.prototype.endVariables = function () {
                this.lastVariables = null;
                return this;
            };
            ComponentBuilder.prototype.endAll = function () {
                this.endConstraint();
                this.endVariables();
                return this;
            };
            ComponentBuilder.prototype.method = function (signature, fn, lift) {
                if (lift === void 0) { lift = true; }
                if (!this.lastConstraint) {
                    console.error('Builder function "method" called with no constraint');
                    return this;
                }
                var p = parseActivation('method', signature);
                if (p == null) {
                    return this;
                }
                // helper function to make sure variable belongs to constraint
                var constraintVars = this.lastConstraint.variables;
                var isNotConstraintVar = function (name, i) {
                    if ((!p.priorFlags || !p.priorFlags[i]) &&
                        constraintVars.indexOf(name) < 0) {
                        console.error("Variable " + name +
                            " does not belong to constraint in method " + signature);
                        return true;
                    }
                    else {
                        return false;
                    }
                };
                if (u.multiArray.some(p.inputs, isNotConstraintVar) ||
                    u.multiArray.some(p.outputs, isNotConstraintVar)) {
                    return this;
                }
                u.arraySet.addKnownDistinct(this.lastConstraint.methods, {
                    inputs: p.inputs,
                    priorFlags: p.priorFlags,
                    outputs: p.outputs,
                    fn: lift ? r.liftFunction(fn, p.promiseFlags) : fn
                });
                return this;
            };
            /*----------------------------------------------------------------
             * Change optional policy on last constraint or variable.
             */
            ComponentBuilder.prototype.optional = function (opt) {
                if (opt === void 0) { opt = 1 /* Max */; }
                if (this.lastConstraint) {
                    this.lastConstraint.optional = opt;
                }
                else if (this.lastVariables) {
                    if (Array.isArray(this.lastVariables)) {
                        var a = this.lastVariables;
                        for (var i = 0, l = a.length; i < l; ++i) {
                            a[i].optional = opt;
                        }
                    }
                    else {
                        this.lastVariables.optional = opt;
                    }
                }
                else {
                    console.error("Call to optional must follow constraint or variable");
                }
                return this;
            };
            /*----------------------------------------------------------------
             */
            ComponentBuilder.prototype.command = function (loc, signature, fn, lift, sync) {
                if (lift === void 0) { lift = true; }
                if (sync === void 0) { sync = false; }
                this.endAll();
                if (this.invalidLoc(loc)) {
                    return this;
                }
                var p = parseActivation('command', signature);
                if (p == null) {
                    return this;
                }
                if (u.multiArray.some(p.inputs, invalidPath) ||
                    u.multiArray.some(p.outputs, invalidPath)) {
                    return this;
                }
                this.target.commands.push({
                    loc: loc,
                    inputs: p.inputs,
                    priorFlags: p.priorFlags,
                    outputs: p.outputs,
                    fn: lift ? r.liftFunction(fn, p.promiseFlags) : fn,
                    synchronous: sync
                });
                return this;
            };
            //--------------------------------------------
            ComponentBuilder.prototype.syncommand = function (loc, signature, fn, lift) {
                if (lift === void 0) { lift = true; }
                return this.command(loc, signature, fn, lift, true);
            };
            /*----------------------------------------------------------------
             * Add output designation
             */
            ComponentBuilder.prototype.output = function (variable) {
                this.endAll();
                if (invalidPath(variable)) {
                    return this;
                }
                this.target.outputs.push({ variable: variable });
                return this;
            };
            /*----------------------------------------------------------------
             * Convenience method for many outputs at once
             */
            ComponentBuilder.prototype.outputs = function (variables) {
                variables.trim().split(/\s*,\s*/).forEach(this.output, this);
                return this;
            };
            ComponentBuilder.prototype.touchDep = function () {
                this.endAll();
                var from, to;
                if (arguments.length == 1) {
                    var split = arguments[0].trim().split(/\s*=>\s*/);
                    if (split.length == 2) {
                        from = split[0];
                        to = split[1];
                    }
                    else {
                        console.error('Invalid touch dependency signature: "' + arguments[0] + '"');
                        return this;
                    }
                }
                else {
                    from = arguments[0];
                    to = arguments[1];
                }
                if (invalidPath(from) || invalidPath(to)) {
                    return this;
                }
                this.target.touchDeps.push({ from: from, to: to });
                return this;
            };
            /*----------------------------------------------------------------
             * Build constraint represented by simple equation.
             */
            ComponentBuilder.prototype.equation = function (eqString) {
                this.endAll();
                // Parse the equation
                try {
                    var equation = model.eqn.parse(eqString);
                    // Check variables
                    var varNames = Object.keys(equation.vars);
                    if (varNames.some(invalidPath)) {
                        return this;
                    }
                    // Create constraint spec
                    var cspec = {
                        variables: varNames,
                        methods: [],
                        optional: 0 /* Default */
                    };
                    var outName;
                    var notOutput = function (name) {
                        return name !== outName;
                    };
                    for (var i = 0, l = varNames.length; i < l; ++i) {
                        outName = varNames[i];
                        var inNames;
                        var priorFlags = undefined;
                        if (equation.op === '==') {
                            inNames = varNames.filter(notOutput);
                        }
                        else {
                            inNames = varNames;
                            priorFlags = [];
                            priorFlags[i] = true;
                        }
                        // Make signature
                        var signature = inNames.join(',') + '->' + outName;
                        // Build method function
                        var fn = model.eqn.makeFunction(inNames, outName, equation);
                        // Create method spec
                        var mspec = {
                            inputs: inNames,
                            outputs: [outName],
                            priorFlags: priorFlags,
                            fn: r.liftFunction(fn)
                        };
                        // Add method to constraint
                        u.arraySet.addKnownDistinct(cspec.methods, mspec);
                    }
                    // Record constraint
                    this.target.constraints.push(cspec);
                }
                catch (e) {
                    console.error(e);
                }
                return this;
            };
            /*----------------------------------------------------------------
             * Test for invalid property name
             */
            ComponentBuilder.prototype.invalidLoc = function (loc) {
                if (!loc.match(/^[a-zA-Z][\w$]*$/)) {
                    console.error('Invalid component property name: "' + loc + '"');
                    return true;
                }
                if (this.usedLocs[loc]) {
                    console.error('Cannot redefine component property: "' + loc + '"');
                    return true;
                }
                return false;
            };
            return ComponentBuilder;
        }());
        model.ComponentBuilder = ComponentBuilder;
        ComponentBuilder.prototype['v'] = ComponentBuilder.prototype.variable;
        ComponentBuilder.prototype['vs'] = ComponentBuilder.prototype.variables;
        ComponentBuilder.prototype['n'] = ComponentBuilder.prototype.nested;
        ComponentBuilder.prototype['r'] = ComponentBuilder.prototype.reference;
        ComponentBuilder.prototype['rs'] = ComponentBuilder.prototype.references;
        ComponentBuilder.prototype['c'] = ComponentBuilder.prototype.constraint;
        ComponentBuilder.prototype['m'] = ComponentBuilder.prototype.method;
        ComponentBuilder.prototype['cmd'] = ComponentBuilder.prototype.command;
        ComponentBuilder.prototype['o'] = ComponentBuilder.prototype.output;
        ComponentBuilder.prototype['os'] = ComponentBuilder.prototype.outputs;
        ComponentBuilder.prototype['td'] = ComponentBuilder.prototype.touchDep;
        ComponentBuilder.prototype['eq'] = ComponentBuilder.prototype.equation;
        /*==================================================================
         * Test for invalid variable path
         */
        var pathRegEx = /^[a-zA-Z][\w$]*(\.[a-zA-Z][\w$]*|\[\s*(\d+|\*)\s*\]|\[\s*(\d+\s*)?[a-zA-Z][\w$]*\s*([+-]\s*\d+\s*)?\])*$/;
        function invalidPath(path) {
            if (!path.match(pathRegEx)) {
                console.error('Invalid variable path: "' + path + '"');
                return true;
            }
            return false;
        }
        /*==================================================================
         */
        function parseConstraint(signature) {
            var touchVars;
            var constraintVars;
            var leftRight = signature.trim().split(/\s*=>\s*/);
            if (leftRight.length == 1) {
                constraintVars = leftRight[0].split(/\s*,\s*/);
            }
            else if (leftRight.length == 2) {
                touchVars = leftRight[0].split(/\s*,\s*/);
                constraintVars = leftRight[1].split(/\s*,\s*/);
            }
            else {
                console.error('Invalid constraint signature: "' + signature + '"');
                return null;
            }
            return { touchVars: touchVars,
                constraintVars: constraintVars };
        }
        /*==================================================================
         */
        function parseActivation(description, signature) {
            var leftRight = signature.trim().split(/\s*->\s*/);
            if (leftRight.length != 2) {
                console.error('Invalid ' + description + ' signature: "' + signature + '"');
                return null;
            }
            var inputs = leftRight[0] == '' ? [] : leftRight[0].split(/\s*,\s*/);
            var outputs = leftRight[1] == '' ? [] : leftRight[1].split(/\s*,\s*/);
            var promiseFlags = [];
            var priorFlags = [];
            for (var i = 0, l = inputs.length; i < l; ++i) {
                var stripResult = strip(inputs[i], ['*', '!']);
                if (!stripResult) {
                    return null;
                }
                inputs[i] = stripResult['name'];
                if (stripResult['*']) {
                    promiseFlags[i] = true;
                }
                if (stripResult['!']) {
                    priorFlags[i] = true;
                }
            }
            return { inputs: inputs,
                promiseFlags: promiseFlags.length == 0 ? null : promiseFlags,
                priorFlags: priorFlags.length == 0 ? null : priorFlags,
                outputs: outputs.length == 1 ? outputs : [outputs]
            };
        }
        model.tokensRegEx = /([\w$.]+(\[[^\]]*\][\w$.]*)*|\S)/g;
        function parseList(tokens, i, list) {
            do {
                if (tokens[i] === '[') {
                    var sub = [];
                    list.push(sub);
                    i = parseList(tokens, i + 1, sub);
                    if (tokens[i++] !== ']') {
                        throw 'Expecting "]"';
                    }
                }
                else if (tokens[i] === ',' || tokens[i] === ']') {
                    throw 'Unexpected token "' + tokens[i] + '"';
                }
                else {
                    list.push(tokens[i++]);
                }
                var more = false;
                if (tokens[i] === ',') {
                    ++i;
                    more = true;
                }
            } while (more);
            return i;
        }
        model.parseList = parseList;
        /*================================================================
         * Strip one-character prefixes from front of names
         */
        function strip(name, prefixes) {
            var result = {};
            var c = name.charAt(0);
            while (!c.match(/\w/)) {
                if (prefixes.indexOf(c) >= 0) {
                    if (result[c]) {
                        console.error('Duplicate variable prefix: ' + c);
                        return null;
                    }
                    result[c] = true;
                }
                else if (!c.match(/\s/)) {
                    console.error('Invalid variable prefix: ' + c);
                    return null;
                }
                name = name.substring(1);
                c = name.charAt(0);
            }
            result['name'] = name;
            return result;
        }
    })(model = hd.model || (hd.model = {}));
})(hd || (hd = {}));
/*####################################################################
 * The Path class
 */
var hd;
(function (hd) {
    var model;
    (function (model) {
        var u = hd.utility;
        var r = hd.reactive;
        ////////////////////////////////////////////////
        // Array index pattern
        var IndexPattern = /** @class */ (function () {
            function IndexPattern() {
                if (arguments.length == 1) {
                    this.scale = 0;
                    this.offset = arguments[0];
                }
                else {
                    this.scale = arguments[0];
                    this.index = arguments[1];
                    this.offset = arguments[2];
                }
            }
            // Get the result for this pattern at a particular position
            IndexPattern.prototype.apply = function (pos) {
                if (this.scale == 0) {
                    return this.offset;
                }
                else if (pos[this.index] === undefined) {
                    return undefined;
                }
                else {
                    return this.scale * pos[this.index] + this.offset;
                }
            };
            // Inverse of apply: what sub-postion of the current position
            // would result in this index pattern mapping to n?
            IndexPattern.prototype.inverse = function (pos, n) {
                if (this.scale == 0) {
                    return n == this.offset ? pos : undefined;
                }
                var i = (n - this.offset) / this.scale;
                if (Math.floor(i) !== i) {
                    return undefined;
                }
                if (pos[this.index] !== undefined) {
                    return i == pos[this.index] ? pos : undefined;
                }
                pos = u.shallowCopy(pos);
                pos[this.index] = i;
                return pos;
            };
            return IndexPattern;
        }());
        model.IndexPattern = IndexPattern;
        ////////////////////////////////////////////////
        // Observer the property for a specific field
        var PropertyObserver = /** @class */ (function () {
            function PropertyObserver(path, property, legi, pos) {
                this.path = path;
                this.property = property;
                this.legi = legi;
                this.pos = pos;
                this.property.addObserverChangesOnly(this);
            }
            // Observer no longer needed
            PropertyObserver.prototype.destruct = function () {
                this.property.removeObserver(this);
                if (this.child) {
                    this.child.destruct();
                }
            };
            // Property changes
            PropertyObserver.prototype.onNext = function () {
                this.path.onNextProperty(this);
            };
            PropertyObserver.prototype.onError = function () { };
            PropertyObserver.prototype.onCompleted = function () { };
            return PropertyObserver;
        }());
        model.PropertyObserver = PropertyObserver;
        ////////////////////////////////////////////////
        // Observe an entire array
        var ArrayObserver = /** @class */ (function () {
            function ArrayObserver(path, cmp, legi, pos) {
                this.path = path;
                this.cmp = cmp;
                this.legi = legi;
                this.pos = pos;
                // Whether there is just one child or multiple children depends on
                // whether we are observing the entire array or just one element
                this.children = [];
                if (cmp instanceof model.ArrayComponent) {
                    this.cmp.changes.addObserver(this);
                }
            }
            ArrayObserver.prototype.alsoWatchLength = function () {
                this.cmp.$length.addObserver(this, this.onNextLength, null, null);
            };
            // Observer no longer needed
            ArrayObserver.prototype.destruct = function () {
                this.cmp.changes.removeObserver(this);
                this.cmp.$length.removeObserver(this);
                if (this.hasOwnProperty('children')) {
                    for (var i = 0, l = this.children.length; i < l; ++i) {
                        var child = this.children[i];
                        if (child) {
                            child.destruct();
                        }
                    }
                }
            };
            ArrayObserver.prototype.onNext = function (idx) {
                this.path.onNextArray(this, idx);
            };
            ArrayObserver.prototype.onNextLength = function (legnth) {
                this.path.onNextArrayLength(this);
            };
            ArrayObserver.prototype.onError = function () { };
            ArrayObserver.prototype.onCompleted = function () { };
            return ArrayObserver;
        }());
        model.ArrayObserver = ArrayObserver;
        /*==================================================================
         */
        var PathSetIterator = /** @class */ (function () {
            function PathSetIterator(paths) {
                this.paths = paths;
                this.locks = [{}];
                this.pos = this.multiPosition(0, 1);
            }
            PathSetIterator.prototype.next = function () {
                if (this.pos !== null) {
                    this.pos = this.multiPosition(this.paths.length - 1, -1);
                }
            };
            PathSetIterator.prototype.multiPosition = function (idx, dir) {
                for (var l = this.paths.length; idx >= 0 && idx < l; idx += dir) {
                    if (dir > 0) {
                        this.locks[idx + 1] = this.paths[idx].begin(this.locks[idx]);
                    }
                    else {
                        this.locks[idx + 1] = this.paths[idx].next(this.locks[idx], this.locks[idx + 1]);
                    }
                    dir = this.locks[idx + 1] === null ? -1 : 1;
                }
                if (idx == l) {
                    return this.locks[idx];
                }
                else {
                    return null;
                }
            };
            return PathSetIterator;
        }());
        model.PathSetIterator = PathSetIterator;
        /*==================================================================
         * An observable representing a particular property path in a
         * component.
         *
         * Each path can use arbitrary variables.  The path will assign
         * an order to its variables -- i.e., i_0, i_1, i_2, ...
         *
         * Each slice is assigned a unique variable name.  These are
         * considered internal variable names.  They will not be included in
         * any positions published by the path, nor will they be looked for
         * in any positions passed to this path from the outside.  Internal
         * variable names will always come after external variable names in
         * the ordering.
         */
        var Path = /** @class */ (function (_super) {
            __extends(Path, _super);
            /*----------------------------------------------------------------
             * Perform initial search.
             */
            function Path(start, path) {
                var _this = _super.call(this) || this;
                // Break path down into property names and index patterns
                var legs = parse(path);
                if (legs) {
                    _this.start = start;
                    _this.legs = legs;
                }
                else {
                    // Create a path which always just returns undefined
                    _this.start = undefined;
                    _this.legs = [];
                }
                _this.slices = countSlices(_this.legs);
                // Initialize
                _this.rootObserver = _this.createObservers(_this.start, 0, {});
                return _this;
            }
            /*----------------------------------------------------------------
             * We're no longer needed; prune observer tree
             */
            Path.prototype.destruct = function () {
                if (this.rootObserver) {
                    this.rootObserver.destruct();
                    this.rootObserver = undefined;
                }
            };
            /*----------------------------------------------------------------
             * Is this path constant (no references/arrays)?
             */
            Path.prototype.isConstant = function () {
                return this.rootObserver === undefined;
            };
            /*----------------------------------------------------------------
             * Get the current result for a position
             */
            Path.prototype.get = function (pos) {
                if (pos === void 0) { pos = {}; }
                return this.getRec(this.start, 0, pos, this.legs.length);
            };
            /*----------------------------------------------------------------
             * Get the result for a position up to the specified depth.
             */
            Path.prototype.getRec = function (value, legi, pos, limit) {
                if (value === undefined) {
                    return undefined;
                }
                // If we reach the end, return the value
                if (legi >= limit) {
                    return value;
                }
                // Consider current leg
                var leg = this.legs[legi];
                if (typeof leg === 'string' && value != null) {
                    // Field lookup
                    return this.getRec(value[leg], legi + 1, pos, limit);
                }
                else if (leg instanceof IndexPattern) {
                    if (leg.scale == 0 || leg.index in pos) {
                        // Index can be determined: single value
                        var idx = leg.apply(pos);
                        if (idx === undefined) {
                            return undefined;
                        }
                        return this.getRec(value[idx], legi + 1, pos, limit);
                    }
                    else {
                        // Index undetermined: array of all values
                        var results = [];
                        for (var i = 0, l = value.length; i < l; ++i) {
                            var newpos = leg.inverse(pos, i);
                            var result = this.getRec(value[i], legi + 1, newpos, limit);
                            if (result === undefined) {
                                return undefined;
                            }
                            results.push(result);
                        }
                        return results;
                    }
                }
                return undefined;
            };
            /*----------------------------------------------------------------
             * Examine every possible value for this path, creating observers
             * for any references found along the way.
             */
            Path.prototype.createObservers = function (value, legi, pos) {
                if (value === undefined) {
                    return;
                }
                // Iterate until we finish or we find a reference to observe.
                // If we find a reference, we create an observer, recurse to
                //   finish out the rest of the path, and return the observer.
                for (var l = this.legs.length; legi < l; ++legi) {
                    var leg = this.legs[legi];
                    // Field access
                    if (typeof leg === 'string' && value !== null) {
                        if (value instanceof model.Component) {
                            var propname = '$' + leg;
                            if (propname in value) {
                                var prop = value[propname];
                                var pobs = new PropertyObserver(this, prop, legi, pos);
                                pobs.child = this.createObservers(prop.get(), legi + 1, pos);
                                return pobs;
                            }
                        }
                        value = value[leg]; // continue iteration
                    }
                    // Array access
                    else if (leg instanceof IndexPattern &&
                        (value instanceof model.ArrayComponent || value instanceof Array)) {
                        // Constant array access
                        if (leg.scale == 0 || pos[leg.index] !== undefined) {
                            var idx = leg.apply(pos);
                            if (value instanceof model.ArrayComponent) {
                                var aobs = new ArrayObserver(this, value, legi, pos);
                                aobs.children[idx] = this.createObservers(value[idx], legi + 1, pos);
                                return aobs;
                            }
                            value = value[idx]; // continue iteration
                        }
                        // Variable array access
                        else {
                            if (value instanceof model.ArrayComponent) {
                                // Note: an array observer for an Array (as opposed to an ArrayComponent)
                                //       will not actually observe, but it will manage its children
                                var aobs = new ArrayObserver(this, value, legi, pos);
                                for (var j = 0, m = value.length; j < m; ++j) {
                                    if (value[j] !== undefined) {
                                        var n = leg.inverse(pos, j);
                                        if (n !== undefined) {
                                            aobs.children[j] = this.createObservers(value[j], legi + 1, n);
                                        }
                                    }
                                }
                                if (leg.slice) {
                                    aobs.alsoWatchLength();
                                }
                            }
                            return aobs;
                        }
                    }
                    else {
                        break;
                    }
                }
            };
            /*----------------------------------------------------------------
             * Find first position that (1) has a value, and (2) is a
             * superposition of lock.  Lock may be empty to allow all
             * positions.
             */
            Path.prototype.begin = function (lock) {
                return this.beginRec(this.start, 0, lock);
            };
            Path.prototype.beginRec = function (value, legi, lock) {
                // Undefined means no valid positions
                if (value === undefined) {
                    return null;
                }
                // If we reach the end, then the lock indicates a valid position
                if (legi == this.legs.length) {
                    return lock;
                }
                // Consider the current leg
                var leg = this.legs[legi];
                if (typeof leg === 'string' && value !== null) {
                    // Only one possibility for a field
                    return this.beginRec(value[leg], legi + 1, lock);
                }
                else if (leg instanceof IndexPattern &&
                    (value instanceof model.ArrayComponent || value instanceof Array)) {
                    if (leg.slice) {
                        // We need to find the first that works for every element of value
                        return this.multiPosition(value, legi, lock);
                    }
                    else {
                        // Is index variable locked down?
                        if (leg.index in lock) {
                            // Yes:  only one thing we can try
                            var idx = leg.apply(lock);
                            if (idx !== undefined) {
                                return this.beginRec(value[idx], legi + 1, lock);
                            }
                        }
                        else {
                            // No: let's try them all and take the first one that works
                            for (var idx = 0, l = value.length; idx < l; ++idx) {
                                var newlock = leg.inverse(lock, idx);
                                if (newlock !== undefined) {
                                    var found = this.beginRec(value[idx], legi + 1, newlock);
                                    if (found) {
                                        return found;
                                    }
                                }
                            }
                        }
                    }
                }
                // If we make it here, then nothing worked
                return null;
            };
            /*----------------------------------------------------------------
             * Find first position /after/ specified position which is a
             * subposition of lock.  Lock may be empty to allow all
             * positions
             */
            Path.prototype.next = function (lock, pos) {
                return this.nextRec(this.start, 0, lock, pos);
            };
            Path.prototype.nextRec = function (value, legi, lock, pos) {
                // If we run out of values or reach the end, then there's nothing
                //   more that we can do:  there is no next value
                if (value === undefined) {
                    return null;
                }
                if (legi == this.legs.length) {
                    return null;
                }
                // Consider the current leg
                var leg = this.legs[legi];
                if (typeof leg === 'string' && value !== null) {
                    // A field cannot be changed; we'll have to keep looking
                    return this.nextRec(value[leg], legi + 1, lock, pos);
                }
                else if (leg instanceof IndexPattern &&
                    (value instanceof model.ArrayComponent || value instanceof Array)) {
                    if (leg.slice) {
                        // We need to find the next that works for every element of value
                        return this.multiPosition(value, legi, lock, pos);
                    }
                    else {
                        // Figure out which element is currently being used
                        var idx = leg.apply(pos);
                        if (idx === undefined) {
                            return null;
                        } // shouldn't happen --
                        // pos should be a valid position
                        if (leg.index in lock) {
                            // If it's locked, it cannot be changed; we'll have to keep looking
                            return this.nextRec(value[idx], legi + 1, lock, pos);
                        }
                        else {
                            // First see if we can do a next while staying on the current element
                            //   by locking down this index
                            var extLock = u.shallowCopy(lock);
                            extLock[leg.index] = idx;
                            var found = this.nextRec(value[idx], legi + 1, extLock, pos);
                            if (found) {
                                return found;
                            }
                            // If that didn't work, try remaining elements to see if we
                            //   can find something
                            var l;
                            for (++idx, l = value.length; idx < l; ++idx) {
                                var newpos = leg.inverse(lock, idx);
                                if (newpos !== undefined) {
                                    var found = this.beginRec(value[idx], legi + 1, newpos);
                                    if (found) {
                                        return found;
                                    }
                                }
                            }
                        }
                    }
                }
                // If we make it here, then nothing worked
                return null;
            };
            /*----------------------------------------------------------------
             * Find the first/next position which (1) produces valid results
             * for /all/ values, and (2) is a superposition of lock. (If
             * from is specified, it's next; if not, it's first.)
             */
            Path.prototype.multiPosition = function (values, legi, lock, from) {
                var locks, dir;
                // The position at locks[i] is used to query a position from values[i].
                // The position returned becomes the lock for the next value.
                // If dir == 1, then the previous position query worked and we're moving
                //   ahead to try the next one ==> use begin
                // If dir == -1, then the previous position query failed and we're backing
                //   up to try to find a different lock ==> use next
                if (from) {
                    locks = [lock, from];
                    dir = -1;
                }
                else {
                    locks = [lock];
                    dir = 1;
                }
                for (var idx = 0, l = values.length; idx >= 0 && idx < l; idx += dir) {
                    if (dir > 0) {
                        locks[idx + 1] = this.beginRec(values[idx], legi + 1, locks[idx]);
                    }
                    else {
                        locks[idx + 1] = this.nextRec(values[idx], legi + 1, locks[idx], locks[idx + 1]);
                    }
                    dir = locks[idx + 1] === null ? -1 : 1;
                }
                // Either we made it all the way to the end, or we regressed to the beginning
                if (idx == l) {
                    return locks[idx];
                }
                else {
                    return null;
                }
            };
            /*----------------------------------------------------------------
             * Property changed
             */
            Path.prototype.onNextProperty = function (obs) {
                if (obs.child) {
                    obs.child.destruct();
                }
                obs.child = this.createObservers(this.getRec(this.start, 0, obs.pos, obs.legi + 1), obs.legi + 1, obs.pos);
                this.sendNext(obs.pos);
            };
            /*----------------------------------------------------------------
             * Array changed
             */
            Path.prototype.onNextArray = function (obs, idx) {
                var leg = this.legs[obs.legi];
                var newpos = leg.inverse(obs.pos, idx);
                if (newpos !== undefined) {
                    if (obs.children[idx]) {
                        obs.children[idx].destruct();
                    }
                    obs.children[idx] = this.createObservers(this.getRec(this.start, 0, newpos, obs.legi + 1), obs.legi + 1, newpos);
                    this.sendNext(newpos);
                }
            };
            /*----------------------------------------------------------------
             * Array size changed (only for slices)
             */
            Path.prototype.onNextArrayLength = function (obs) {
                this.sendNext(obs.pos);
            };
            return Path;
        }(r.BasicObservable));
        model.Path = Path;
        /*==================================================================
         * Parse path into legs
         */
        function parse(pathstr) {
            var s = pathstr;
            var nonempty = /\S/;
            var field = /^\s*\.?([a-zA-Z][\w$]*)/;
            var cindex = /^\s*\[\s*(\d+)\s*\]/;
            var vindex = /^\s*\[\s*(\d+)?\s*(\*|[a-zA-Z][\w$]*)\s*(?:([+-])\s*(\d+)\s*)?\]/;
            var legs = [];
            var temps = 0;
            while (nonempty.test(s)) {
                var m;
                if (m = field.exec(s)) {
                    legs.push(m[1]);
                }
                else if (m = cindex.exec(s)) {
                    legs.push(new IndexPattern(Number(m[1])));
                }
                else if (m = vindex.exec(s)) {
                    var index;
                    var slice;
                    if (m[2] == '*') {
                        index = '#' + temps++;
                        slice = true;
                    }
                    else {
                        index = m[2];
                        slice = false;
                    }
                    var scale = 1;
                    var offset = 0;
                    if (m[1]) {
                        scale = Number(m[1]);
                    }
                    if (m[4]) {
                        offset = Number(m[4]);
                        if (m[3] == '-') {
                            offset = -offset;
                        }
                    }
                    var pat = new IndexPattern(scale, index, offset);
                    if (slice) {
                        pat.slice = true;
                    }
                    legs.push(pat);
                }
                else {
                    console.error('Unable to parse path "' + pathstr + '"');
                    return null;
                }
                s = s.substr(m[0].length);
            }
            return legs;
        }
        model.parse = parse;
        function countDimensions(legs) {
            var dimensions = 0;
            var variables = {};
            for (var i = 0, l = legs.length; i < l; ++i) {
                var leg = legs[i];
                if (leg instanceof IndexPattern &&
                    !leg.slice &&
                    !variables[leg.index]) {
                    ++dimensions;
                    variables[leg.index] = true;
                }
            }
            return dimensions;
        }
        function countSlices(legs) {
            var slices = 0;
            for (var i = 0, l = legs.length; i < l; ++i) {
                var leg = legs[i];
                if (leg instanceof IndexPattern && leg.slice) {
                    ++slices;
                }
            }
            return slices;
        }
        /*==================================================================
         */
        var PathValue = /** @class */ (function (_super) {
            __extends(PathValue, _super);
            function PathValue(path) {
                var _this = _super.call(this) || this;
                _this.path = path;
                path.addObserver(_this, _this.onPositionChange, null, null);
                _this.set(path.get({}));
                return _this;
            }
            PathValue.prototype.onPositionChange = function () {
                this.set(this.path.get({}));
            };
            return PathValue;
        }(r.BasicSignal));
        model.PathValue = PathValue;
    })(model = hd.model || (hd.model = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var model;
    (function (model) {
        var r = hd.reactive;
        var Command = /** @class */ (function (_super) {
            __extends(Command, _super);
            function Command(name, fn, inputs, priorFlags, outputs) {
                var _this = _super.call(this) || this;
                // Is this an external operation?  (Does it trigger an update after execution?)
                _this.external = true;
                _this.id = model.makeId(name);
                _this.name = name;
                if (typeof fn === 'function') {
                    _this.fn = fn;
                }
                else {
                    _this.result = fn;
                }
                _this.inputs = inputs;
                _this.priorFlags = priorFlags;
                _this.outputs = outputs;
                return _this;
            }
            Command.prototype.onNext = function () {
                this.sendNext(this);
            };
            Command.prototype.onError = function () { };
            Command.prototype.onCompleted = function () { };
            return Command;
        }(r.BasicObservable));
        model.Command = Command;
        Command.prototype.activate = Command.prototype.onNext;
        var None = /** @class */ (function (_super) {
            __extends(None, _super);
            function None(observables) {
                var _this = _super.call(this) || this;
                _this.cache = [];
                _this.observables = observables;
                _this.cache.length = observables.length;
                var count = 0;
                for (var i = 0, l = observables.length; i < l; ++i) {
                    observables[i].addObserver(_this, _this.onNext, null, null, count++);
                }
                return _this;
            }
            None.prototype.onNext = function (value, count) {
                this.cache[count] = value ? true : false;
                var none = true;
                for (var i = 0, l = this.cache.length; i < l; ++i) {
                    if (this.cache[i] === undefined) {
                        return;
                    }
                    none = none && !this.cache[i];
                }
                this.set(none);
            };
            return None;
        }(r.BasicSignal));
        model.None = None;
        var SynchronousCommand = /** @class */ (function (_super) {
            __extends(SynchronousCommand, _super);
            function SynchronousCommand(name, fn, inputs, usePriors, outputs) {
                var _this = _super.call(this, name, fn, inputs, usePriors, outputs) || this;
                var properties = [];
                for (var i = 0, l = inputs.length; i < l; ++i) {
                    var vv = inputs[i];
                    if (vv instanceof model.Variable) {
                        properties.push(vv.pending);
                        properties.push(vv.error);
                    }
                }
                _this.ready = new None(properties);
                return _this;
            }
            SynchronousCommand.prototype.onNext = function () {
                if (this.ready.get()) {
                    this.sendNext(this);
                }
            };
            return SynchronousCommand;
        }(Command));
        model.SynchronousCommand = SynchronousCommand;
        SynchronousCommand.prototype.activate = SynchronousCommand.prototype.onNext;
        function varValue(vv) {
            return vv.value.get();
        }
    })(model = hd.model || (hd.model = {}));
})(hd || (hd = {}));
/*####################################################################
 * The ArrayComponent class.
 */
var hd;
(function (hd) {
    var model;
    (function (model) {
        var u = hd.utility;
        var r = hd.reactive;
        /*==================================================================
         * The ArrayComponent class.
         */
        var ArrayComponent = /** @class */ (function (_super) {
            __extends(ArrayComponent, _super);
            /*----------------------------------------------------------------
             */
            function ArrayComponent(elementType) {
                var _this = _super.call(this) || this;
                _this.elementType = elementType;
                // The actual array for storing contents
                _this.elements = [];
                // The length of the array; this controls how many properties are defined
                _this.$length = new r.BasicSignal(0);
                // Observable for changes to the array
                _this.changes = new r.BasicObservable();
                _this.scheduled = false;
                return _this;
            }
            /*----------------------------------------------------------------
             * Length getter
             */
            ArrayComponent.prototype.getLength = function () { return this.$length.get(); };
            /*----------------------------------------------------------------
             * Length setter
             */
            ArrayComponent.prototype.setLength = function (n) {
                var curlen = this.$length.get();
                // If we're decreasing length
                if (n < curlen) {
                    for (var i = curlen - 1; i >= n; --i) {
                        if (this.elements[i] !== undefined) {
                            this.elements[i] = undefined;
                            this.changes.sendNext(i);
                            this.scheduleNext();
                        }
                    }
                }
                // If we're increasing length, define properties for new indices
                else {
                    for (var i = curlen; i < n; ++i) {
                        if (!ArrayComponent.prototype.hasOwnProperty(i.toString())) {
                            Object.defineProperty(ArrayComponent.prototype, i.toString(), {
                                configurable: false,
                                enumerable: true,
                                get: getter(i),
                                set: setter(i)
                            });
                        }
                    }
                }
                this.elements.length = n;
                this.$length.set(n);
            };
            /*----------------------------------------------------------------
             * Element getter
             */
            ArrayComponent.prototype.get = function (i) {
                return this.elements[i];
            };
            /*----------------------------------------------------------------
             * Element setter
             */
            ArrayComponent.prototype.set = function (i, v) {
                // Ensure length is big enough to hold this
                if (this.$length.get() <= i) {
                    this.setLength(i + 1);
                }
                if (this.elements[i] !== undefined || v !== undefined) {
                    this.elements[i] = v;
                    this.changes.sendNext(i);
                    this.scheduleNext();
                }
            };
            /*----------------------------------------------------------------
             * Push operation
             */
            ArrayComponent.prototype.push = function (v) {
                var i = this.$length.get();
                this.set(i, v);
            };
            ArrayComponent.prototype.expand = function (desc, start) {
                if (start === void 0) { start = this.$length.get(); }
                var count, inits;
                if (typeof desc === 'number') {
                    count = desc;
                    inits = [];
                }
                else if (Array.isArray(desc)) {
                    inits = desc;
                    count = desc.length;
                }
                else {
                    inits = [desc];
                    count = 1;
                }
                if (count > 0) {
                    // Set length
                    var oldLength = this.getLength();
                    var newLength = oldLength + count;
                    this.setLength(newLength);
                    // Copy old stuff foward
                    for (var i = oldLength - 1; i >= start; --i) {
                        this.set(i + count, this.elements[i]);
                    }
                    // Initialize new spaces
                    if (this.elementType) {
                        var klass;
                        var spec;
                        if ((typeof this.elementType) == 'function') {
                            klass = this.elementType;
                        }
                        else {
                            klass = model.Component;
                            spec = this.elementType;
                        }
                        for (var i = 0, l = count; i < l; ++i) {
                            var cmp;
                            if (klass === model.Variable) {
                                cmp = new model.Variable("el" + i, inits[i]);
                            }
                            else {
                                cmp = new klass();
                                if (cmp instanceof ArrayComponent) {
                                    if (inits[i] !== undefined) {
                                        cmp.expand(inits[i], 0);
                                    }
                                }
                                else if (spec) {
                                    model.Component.construct(cmp, spec, inits[i]);
                                }
                            }
                            this.set(start + i, cmp);
                            model.Component.claim(this, cmp);
                        }
                    }
                    else {
                        for (var i = start, l = start + count; i < l; ++i) {
                            this.set(i, undefined);
                        }
                    }
                }
            };
            /*----------------------------------------------------------------
             */
            ArrayComponent.prototype.collapse = function (count, start) {
                if (start === void 0) { start = this.$length.get() - count; }
                if (count > 0) {
                    var oldLength = this.getLength();
                    var newLength = oldLength - count;
                    // Destruct existing spaces
                    for (var i = start, l = start + count; i < l; ++i) {
                        var cmp = this.elements[i];
                        if (cmp !== undefined &&
                            model.Component.release(this, cmp) &&
                            cmp instanceof model.Component) {
                            model.Component.destruct(cmp);
                        }
                    }
                    // Copy old stuff backward
                    for (var i = start; i < newLength; ++i) {
                        this.set(i, this.elements[i + count]);
                    }
                    // Set length
                    this.setLength(newLength);
                }
            };
            /*----------------------------------------------------------------
             */
            ArrayComponent.prototype.move = function (destination, source, count) {
                if (count === void 0) { count = 1; }
                if (destination > this.elements.length - 1) {
                    destination = this.elements.length - 1;
                }
                if (destination < 0) {
                    destination = 0;
                }
                if (source > this.elements.length - count) {
                    source = this.elements.length - count;
                }
                if (source < 0) {
                    source = 0;
                }
                if (count > this.elements.length) {
                    count = this.elements.length;
                }
                if (count < 0 || destination == source) {
                    return;
                }
                if (destination < source) {
                    var a = destination;
                    var b = source;
                    var c = source + count;
                }
                else {
                    var a = source;
                    var b = source + count;
                    var c = destination + count;
                }
                var s = this.elements.slice(a, b);
                var t = this.elements.slice(b, c);
                Array.prototype.splice.apply(this.elements, [a, t.length].concat(t));
                Array.prototype.splice.apply(this.elements, [a + t.length, s.length].concat(s));
                for (var i = a; i < c; ++i) {
                    this.changes.sendNext(i);
                }
                this.scheduleNext();
            };
            /*----------------------------------------------------------------
             */
            ArrayComponent.prototype.addObserver = function (observer) {
                if (this.observers) {
                    this.observers.push(observer);
                }
                else {
                    this.observers = [observer];
                }
                observer.onNext(this);
            };
            ArrayComponent.prototype.removeObserver = function (observer) {
                if (this.observers) {
                    this.observers = this.observers.filter(function (ver) {
                        return observer !== ver;
                    });
                    if (this.observers.length == 0) {
                        this.observers = undefined;
                    }
                }
            };
            ArrayComponent.prototype.scheduleNext = function () {
                if (!this.scheduled) {
                    this.scheduled = true;
                    u.schedule(r.SignalPriority, this.sendNext, this);
                }
            };
            ArrayComponent.prototype.sendNext = function () {
                this.scheduled = false;
                if (this.observers) {
                    for (var i = 0, l = this.observers.length; i < l; ++i) {
                        if (this.observers[i].onNext) {
                            this.observers[i].onNext(this);
                        }
                    }
                }
            };
            return ArrayComponent;
        }(model.Component));
        model.ArrayComponent = ArrayComponent;
        Object.defineProperty(ArrayComponent.prototype, "length", { configurable: false,
            enumerable: false,
            get: ArrayComponent.prototype.getLength,
            set: ArrayComponent.prototype.setLength });
        // helper - Create getter for specified index
        function getter(i) {
            return function () { return this.get(i); };
        }
        // helper - Create setter for specified index
        function setter(i) {
            return function (v) { this.set(i, v); };
        }
    })(model = hd.model || (hd.model = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var m = hd.model;
    // shortcuts for arrays
    function arrayOf(elementType) {
        return m.ArrayComponent.bind(null, elementType);
    }
    hd.arrayOf = arrayOf;
    hd.array = m.ArrayComponent;
    // Exports
    hd.Variable = m.Variable;
    hd.Constraint = m.Constraint;
    hd.Method = m.Method;
    hd.Component = m.Component;
    hd.ArrayComponent = m.ArrayComponent;
    hd.MaxOptional = 1 /* Max */;
    hd.MinOptional = 2 /* Min */;
    hd.ComponentBuilder = m.ComponentBuilder;
})(hd || (hd = {}));
//
var hd;
(function (hd) {
    var dfa;
    (function (dfa) {
        var u = hd.utility;
        var g = hd.graph;
        var m = hd.model;
        /*==================================================================
         * A record for a single composite-constraint.
         * Records what constraints it combines, and defines methods in
         * terms of combinations of primitive methods.
         */
        var CompositeConstraint = /** @class */ (function () {
            /*----------------------------------------------------------------
             * Constructor
             * Creates a costraint composed from specified primitive
             * constraints.  The constraint is created empty (no methods).
             */
            function CompositeConstraint(cids) {
                // Mapping from composite-method id to ids of primitive methods that compose it
                this.compmids = {};
                this.cids = cids;
                this.id = newCompositeConstraintId(this.cids);
            }
            /*----------------------------------------------------------------
             * Factory method: creates a constraint composed of only a single
             * primitive constraint
             */
            CompositeConstraint.fromPrimitive = function (cgraph, cid) {
                var cmpc = new CompositeConstraint([cid]);
                cgraph.methodsForConstraint(cid).forEach(cmpc.addMethod, cmpc);
                return cmpc;
            };
            /*----------------------------------------------------------------
             * Get the ids of all composite-methods in this constraint
             */
            CompositeConstraint.prototype.getMethodIds = function () {
                return Object.keys(this.compmids);
            };
            /*----------------------------------------------------------------
             * Given a compoiste-method id, return the primitive method ids
             * that compose it.
             */
            CompositeConstraint.prototype.getPrimitiveIdsForMethod = function (mid) {
                return this.compmids[mid];
            };
            CompositeConstraint.prototype.addMethod = function (mids) {
                if (!Array.isArray(mids)) {
                    mids = [mids];
                }
                var mmid = newCompositeMethodId(mids);
                this.compmids[mmid] = mids;
            };
            return CompositeConstraint;
        }());
        dfa.CompositeConstraint = CompositeConstraint;
        /*==================================================================
         */
        function addCompositeMethods(compcgraph, fullcgraph, composite) {
            var vids = {};
            compcgraph.variables().forEach(function (vid) {
                u.stringSet.add(vids, vid);
            });
            for (var compmid in composite.compmids) {
                var mids = composite.compmids[compmid];
                var outs = {};
                for (var i = 0, l = mids.length; i < l; ++i) {
                    var mid = mids[i];
                    outs = fullcgraph.outputsForMethod(mid).reduce(u.stringSet.build, outs);
                }
                var ins = u.stringSet.difference(vids, outs);
                compcgraph.addMethod(compmid, composite.id, u.stringSet.members(ins), u.stringSet.members(outs));
            }
        }
        dfa.addCompositeMethods = addCompositeMethods;
        /*==================================================================
         */
        function makeConstraintGraph(cgraph, composite) {
            var compcgraph = new g.CachingConstraintGraph();
            cgraph.variables().forEach(compcgraph.addVariable, compcgraph);
            addCompositeMethods(compcgraph, cgraph, composite);
            return compcgraph;
        }
        dfa.makeConstraintGraph = makeConstraintGraph;
        /*==================================================================
         */
        function composeAllConstraints(cgraph) {
            var composites = cgraph.constraints()
                .filter(g.isNotStayConstraint)
                .map(CompositeConstraint.fromPrimitive.bind(null, cgraph));
            while (composites.length > 1) {
                var newcomposites = [];
                for (var i = 0, l = composites.length; i < l; i += 2) {
                    if (i + 1 < l) {
                        newcomposites.push(addConstraints(composites[i], composites[i + 1], cgraph));
                    }
                    else {
                        newcomposites.push(composites[i]);
                    }
                }
                composites = newcomposites;
            }
            return composites[0];
        }
        dfa.composeAllConstraints = composeAllConstraints;
        /*==================================================================
         * Take two composite-constraints and combine them, creating a new
         * composite constraint containing every valid combination of the
         * original methods.
         */
        function addConstraints(compA, compB, cgraph) {
            // Calculate all the individual constraints going into this multi-constraint
            var allcids = u.arraySet.unionKnownDisjoint(compA.cids, compB.cids);
            var compC = new CompositeConstraint(allcids);
            var compmidsA = compA.getMethodIds();
            var compmidsB = compB.getMethodIds();
            // Every combination of method from A with method from B
            for (var a = compmidsA.length - 1; a >= 0; --a) {
                var compmidA = compmidsA[a];
                for (var b = compmidsB.length - 1; b >= 0; --b) {
                    var compmidB = compmidsB[b];
                    // Calculate all the individual methods which would be combined
                    var allmids = u.arraySet.unionKnownDisjoint(compA.getPrimitiveIdsForMethod(compmidA), compB.getPrimitiveIdsForMethod(compmidB));
                    // Make a graph of all the methods
                    var mgraph = new g.Digraph();
                    cgraph.variables().forEach(mgraph.addNode, mgraph);
                    allmids.forEach(function (mid) {
                        var inputs = cgraph.inputsForMethod(mid);
                        var outputs = cgraph.outputsForMethod(mid);
                        mgraph.addNode(mid);
                        inputs.forEach(mgraph.addEdgeTo(mid));
                        outputs.forEach(mgraph.addEdgeFrom(mid));
                    });
                    // Check the graph to see if this is valid combination
                    if (isValidCompositeMethod(mgraph, allmids.reduce(u.stringSet.build, {}))) {
                        compC.addMethod(allmids);
                    }
                }
            }
            return compC;
        }
        /*==================================================================
         * Test a graph to make sure it is a valid multi-method.
         * - no two methods with the same output
         * - no cycles
         */
        function isValidCompositeMethod(mgraph, mids) {
            var indegree = {};
            var sources = [];
            var nodes = mgraph.getNodes();
            // Note: To test for cycles we basically do a topological sort of
            //       the nodes in the graph without remembering the result; we
            //       only care whether it worked or not.
            //
            //       Rather than actually removing nodes from the graph, we
            //       keep separate counters of how many edges are going into
            //       each node.
            // Initialize the counter for each node
            // Nodes with a count of zero are added to the sources list
            // If any variables have a count > 1, the graph is invalid
            //   (means more than one method is writing to the variable)
            var methodOutputConflict = false;
            nodes.forEach(function (id) {
                var count = mgraph.getInsFor(id).length;
                indegree[id] = count;
                if (count == 0) {
                    sources.push(id);
                }
                else if (count > 1 && !mids[id]) {
                    methodOutputConflict = true;
                }
            });
            if (methodOutputConflict) {
                return false;
            }
            // We "remove" source nodes by decrementing the counters of each
            // of its output variables
            var removeInput = function removeInput(nodeId) {
                if (--indegree[nodeId] == 0) {
                    sources.push(nodeId);
                }
            };
            var numvisited = 0;
            while (sources.length > 0) {
                var id = sources.pop();
                ++numvisited;
                mgraph.getOutsFor(id).forEach(removeInput);
            }
            // If we visited every node, then we have a topological ordering
            return (numvisited == nodes.length);
        }
        /*==================================================================
         * Create a unique descriptive id for a multi-constraint
         */
        function newCompositeConstraintId(cids) {
            var name = 'composite';
            if (m.debugIds) {
                var idparts = [];
                cids.forEach(function (cid) {
                    if (idparts.length != 0) {
                        idparts.push(' + ');
                    }
                    idparts.push('(', cid.substring(0, cid.indexOf('#')), ')');
                });
                name = idparts.join('');
            }
            return m.makeId(name);
        }
        /*==================================================================
         * Create a unique descriptive id for a multi-method
         */
        function newCompositeMethodId(mids) {
            var name = 'composite';
            if (m.debugIds) {
                var idparts = [];
                mids.forEach(function (mid) {
                    if (idparts.length != 0) {
                        idparts.push(' + ');
                    }
                    idparts.push('(', mid.substring(0, mid.indexOf('#')), ')');
                });
                name = idparts.join('');
            }
            return m.makeId(name);
        }
    })(dfa = hd.dfa || (hd.dfa = {}));
})(hd || (hd = {}));
/*####################################################################
 */
var hd;
(function (hd) {
    var dfa;
    (function (dfa_1) {
        var u = hd.utility;
        /*==================================================================
         * The interface for a DFA
         */
        // Defines order of compile
        var Order;
        (function (Order) {
            Order[Order["Low"] = 0] = "Low";
            Order[Order["High"] = 1] = "High";
        })(Order = dfa_1.Order || (dfa_1.Order = {}));
        ;
        /*==================================================================
         * A DFA in which nodes are simply unique strings.  Makes debugging
         * easy.  Required for compilation.
         *
         * Leaf nodes are simply the leaf value; ensures that leafs with the
         * same value use the same node.
         */
        var SoftLinkedDfa = /** @class */ (function () {
            function SoftLinkedDfa() {
                // Transition table for each node
                this.transitions = {};
                // Since all nodes are strings, we need a separate flag to keep
                // track of the leafs
                this.leafs = {};
                // Pointer to root node
                this.root = null;
                // How many nodes we have (used to generate unique strings)
                this.count = 0;
            }
            /*----------------------------------------------------------------
             * Create/return new node with given transition table
             */
            SoftLinkedDfa.prototype.addNode = function (transitions) {
                var id = 'n' + this.count++;
                if (this.count % 1000 == 0) {
                    u.console.compile.error(this.count + ' nodes...');
                }
                this.transitions[id] = transitions;
                return id;
            };
            /*----------------------------------------------------------------
             * Get list of all nodes
             */
            SoftLinkedDfa.prototype.getNodes = function () {
                return Object.keys(this.transitions);
            };
            /*----------------------------------------------------------------
             * Get transition table for a particular node
             */
            SoftLinkedDfa.prototype.getTransitions = function (node) {
                return this.transitions[node];
            };
            /*----------------------------------------------------------------
             * Set which node is the root (must be a node already in the DFA)
             */
            SoftLinkedDfa.prototype.setRoot = function (node) {
                this.root = node;
            };
            /*----------------------------------------------------------------
             * Get the root node
             */
            SoftLinkedDfa.prototype.getRoot = function () {
                return this.root;
            };
            /*----------------------------------------------------------------
             * Create/return new node with given leaf value
             */
            SoftLinkedDfa.prototype.addLeaf = function (leaf) {
                this.leafs[leaf] = true;
                return leaf;
            };
            /*----------------------------------------------------------------
             * Get value if parameter is leaf node; null if not a leaf
             */
            SoftLinkedDfa.prototype.getLeafValue = function (node) {
                return this.leafs[node] ? node : null;
            };
            return SoftLinkedDfa;
        }());
        dfa_1.SoftLinkedDfa = SoftLinkedDfa;
        var HardLinkedDfa = /** @class */ (function () {
            function HardLinkedDfa() {
                // List of all nodes
                this.nodes = [];
                // Pointer to root node
                this.root = null;
            }
            /*----------------------------------------------------------------
             * Create/return new node with given transition table
             */
            HardLinkedDfa.prototype.addNode = function (transitions) {
                this.nodes.push(transitions);
                return transitions;
            };
            /*----------------------------------------------------------------
             * Get list of all nodes
             */
            HardLinkedDfa.prototype.getNodes = function () {
                return this.nodes;
            };
            /*----------------------------------------------------------------
             * Get transition table for a particular node
             */
            HardLinkedDfa.prototype.getTransitions = function (node) {
                return node;
            };
            /*----------------------------------------------------------------
             * Set which node is the root (must be a node already in the DFA)
             */
            HardLinkedDfa.prototype.setRoot = function (node) {
                this.root = node;
            };
            /*----------------------------------------------------------------
             * Get the root node
             */
            HardLinkedDfa.prototype.getRoot = function () {
                return this.root;
            };
            /*----------------------------------------------------------------
             * Create/return new node with given leaf value
             */
            HardLinkedDfa.prototype.addLeaf = function (leaf) {
                return leaf;
            };
            /*----------------------------------------------------------------
             * Get value if parameter is leaf node; null if not a leaf
             */
            HardLinkedDfa.prototype.getLeafValue = function (node) {
                return (typeof node === 'string') ? node : null;
            };
            return HardLinkedDfa;
        }());
        dfa_1.HardLinkedDfa = HardLinkedDfa;
        /*==================================================================
         * DFA simulation function
         */
        function runDfa(dfa, input) {
            var node = dfa.getRoot();
            var plan;
            var i = dfa.order == Order.High ? input.length - 1 : 0;
            var di = dfa.order == Order.High ? -1 : 1;
            while (node && !(plan = dfa.getLeafValue(node))) {
                var nextNode = dfa.getTransitions(node)[input[i]];
                if (nextNode) {
                    node = nextNode;
                }
                i += di;
            }
            return plan ? plan : null;
        }
        dfa_1.runDfa = runDfa;
    })(dfa = hd.dfa || (hd.dfa = {}));
})(hd || (hd = {}));
/*####################################################################
 */
var hd;
(function (hd) {
    var dfa;
    (function (dfa_2) {
        var u = hd.utility;
        var g = hd.graph;
        /*==================================================================
         * Actual compilation function.  Sets up and runs builder.
         */
        function compileToDfa(dfa, cgraph, order) {
            // We're only interested in required constraints
            var cids = cgraph.constraints().filter(g.isNotStayConstraint);
            // Assert: all the constraints have been composed to a single constraint
            if (cids.length != 1) {
                console.error('Cannot compile a constraint graph to a decision tree unless it consists of a single constraint');
                return;
            }
            // Now we know the plans are just the methods of our one constraint
            // Map each plan to the stay constraints for its inputs
            var planInputs = {};
            var mids = cgraph.methodsForConstraint(cids[0]);
            for (var i = 0, l = mids.length; i < l; ++i) {
                var mid = mids[i];
                var midInputs = cgraph.inputsForMethod(mid);
                planInputs[mid] = midInputs.map(g.stayConstraint);
            }
            // Create list of all stay constraints
            var dfaInputs = cgraph.variables().map(g.stayConstraint);
            // Build the DFA
            var builder = new DfaBuilder(planInputs, dfaInputs);
            builder.buildDfa(dfa, order);
        }
        dfa_2.compileToDfa = compileToDfa;
        /*==================================================================
         * DFA building algorithm
         */
        var DfaBuilder = /** @class */ (function () {
            /*----------------------------------------------------------------
             * Initialize
             */
            function DfaBuilder(planInputs, inputs) {
                // Cache of nodes created indexed by path
                this.pathCache = {};
                // Cache of nodes created indexed by transition table
                this.transCache = {};
                // Used for debug output
                this.count = 0;
                this.max = 0;
                this.planInputs = planInputs;
                this.inputs = inputs;
            }
            /*----------------------------------------------------------------
             * Entry point
             */
            DfaBuilder.prototype.buildDfa = function (dfa, order) {
                this.dfa = dfa;
                this.dfa.order = order;
                this.count = 0;
                this.max = this.inputs.length * (this.inputs.length - 1);
                if (order == dfa_2.Order.Low) {
                    dfa.setRoot(this.buildLowTreeNode([], this.inputs));
                }
                else {
                    dfa.setRoot(this.buildHighTreeNode([], this.inputs, Object.keys(this.planInputs)));
                }
            };
            /*----------------------------------------------------------------
             * Attempts to find node with same transition table; if it cannot,
             * then it creates a new one
             */
            DfaBuilder.prototype.supplyNode = function (transitions) {
                // Make signature
                var keys = Object.keys(transitions);
                keys.sort();
                var pieces = [];
                keys.forEach(function (k) {
                    pieces.push(k, transitions[k]);
                });
                var transId = pieces.join(',');
                // Get node
                var node = this.transCache[transId];
                if (node) {
                    return node;
                }
                else {
                    return this.transCache[transId] = this.dfa.addNode(transitions);
                }
            };
            /*----------------------------------------------------------------
             * Build a single tree node for a high-order tree -- and,
             * recursively, all its children.
             */
            DfaBuilder.prototype.buildHighTreeNode = function (encountered, remaining, candidates) {
                var transitions = {};
                // Check to see if we've visited a communatively-equivalent path
                var copy = encountered.slice(0);
                copy.sort();
                var nodeId = copy.join(',');
                var node = this.pathCache[nodeId];
                if (node) {
                    return node;
                }
                var numTransitions = 0;
                // Try each remaining stay constraint
                for (var i = 0, l = remaining.length; i < l; ++i) {
                    // debug output
                    if (encountered.length == 1) {
                        ++this.count;
                        u.console.compile.error(this.count + '/' + this.max);
                    }
                    // We cycle through remaining inputs by shifting them off the front,
                    //   then pushing them on the back when we're done
                    var vid = remaining.shift();
                    // Rule out candidates that output to this variable
                    var reducedCandidates = candidates.filter(function (plan) {
                        return this.planInputs[plan].indexOf(vid) != -1;
                    }, this);
                    // See what we got
                    if (reducedCandidates.length == 1) {
                        transitions[vid] = this.dfa.addLeaf(reducedCandidates[0]);
                        ++numTransitions;
                    }
                    else if (reducedCandidates.length > 0 &&
                        reducedCandidates.length < candidates.length) {
                        encountered.push(vid);
                        transitions[vid] = this.buildHighTreeNode(encountered, remaining, reducedCandidates);
                        encountered.pop();
                        ++numTransitions;
                    }
                    // We're done - push on the back
                    remaining.push(vid);
                }
                // Assertion: should always have at least one transition
                if (numTransitions == 0) {
                    console.error('Building decision tree node with no transitions (we should have already found a solution by this point)');
                }
                // Create node
                return this.pathCache[nodeId] = this.supplyNode(transitions);
            };
            /*----------------------------------------------------------------
             * Build a single tree node for a low-order tree -- and,
             * recursively, all its children.
             */
            DfaBuilder.prototype.buildLowTreeNode = function (encountered, remaining) {
                var transitions = {};
                var numTransitions = 0;
                // Try each remaining stay contraint
                for (var i = 0, l = remaining.length; i < l; ++i) {
                    // debug output
                    if (encountered.length == 1) {
                        ++this.count;
                        u.console.compile.error(this.count + '/' + this.max);
                    }
                    // We cycle through remaining inputs by shifting them off the front,
                    //   then pushing them on the back when we're done
                    var vid = remaining.shift();
                    // See if we can uniquely determine a plan from this information
                    var plan = this.pickPlanFromLow(encountered, remaining);
                    if (plan) {
                        transitions[vid] = this.dfa.addLeaf(plan);
                        ++numTransitions;
                    }
                    else {
                        encountered.push(vid);
                        transitions[vid] = this.buildLowTreeNode(encountered, remaining);
                        encountered.pop();
                        ++numTransitions;
                    }
                    // We're done - push on the back
                    remaining.push(vid);
                }
                // Create node
                return this.supplyNode(transitions);
            };
            /*----------------------------------------------------------------
             * Determine whether the given variables uniquely determine a
             * plan.
             */
            DfaBuilder.prototype.pickPlanFromLow = function (encountered, remaining) {
                var candidates = [];
                // Only consider plans that don't write to any of the remaining variables
                for (var plan in this.planInputs) {
                    if (u.arraySet.isSubset(remaining, this.planInputs[plan])) {
                        candidates.push(plan);
                    }
                }
                // Go through encountered variables in reverse order and
                // rule out candidates that output to the variable
                var i = encountered.length - 1;
                while (i >= 0 && candidates.length > 1) {
                    var newCandidates = candidates.filter(function (plan) {
                        return this.planInputs[plan].indexOf(encountered[i]) != -1;
                    }, this);
                    if (newCandidates.length > 0) {
                        candidates = newCandidates;
                    }
                    --i;
                }
                if (candidates.length == 1) {
                    return candidates[0];
                }
                else {
                    return null;
                }
            };
            return DfaBuilder;
        }());
    })(dfa = hd.dfa || (hd.dfa = {}));
})(hd || (hd = {}));
//
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (b.hasOwnProperty(p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
/*====================================================================
 * A strength assignment tracks the relative strengths of different
 * constraints.  It can be used to compare two constraints, or to get
 * an entire list of all constraints in ascending order.
 *
 * It is assumed that the only constraints being tracked are
 * non-required constraints.  Thus, the strength assignment may assume
 * that any constraint it is not tracking is required.
 */
var hd;
(function (hd) {
    var plan;
    (function (plan) {
        /*==================================================================
         * This strength assignment operates by assigning a numeric value to
         * each constraint.  It is very fast at updating and at comparing
         * two constraints (since you just look up their associated
         * numbers), but slow at generating an ordered list (you would have
         * to sort a list from scratch using the comparison operator).
         */
        var NumericStrengthAssignment = /** @class */ (function () {
            function NumericStrengthAssignment() {
                // Strongest strength assigned so far
                this.strongest = 0;
                // Weakest strength assigned so far
                this.weakest = 0;
                // Map of constraint strengths
                this.strengths = {};
            }
            /*----------------------------------------------------------------
             * Add/re-order individual constraints
             */
            NumericStrengthAssignment.prototype.setToMax = function (cid) {
                this.strengths[cid] = ++this.strongest;
            };
            NumericStrengthAssignment.prototype.setToMin = function (cid) {
                this.strengths[cid] = --this.weakest;
            };
            /*----------------------------------------------------------------
             * Remove an individual constraint
             */
            NumericStrengthAssignment.prototype.remove = function (cid) {
                delete this.strengths[cid];
            };
            /*----------------------------------------------------------------
             * Test whether a constraint is required.
             */
            NumericStrengthAssignment.prototype.isRequired = function (cid) {
                return !(cid in this.strengths);
            };
            /*----------------------------------------------------------------
             */
            NumericStrengthAssignment.prototype.getOptionalsUnordered = function () {
                return Object.keys(this.strengths);
            };
            /*----------------------------------------------------------------
             * Comparison operator
             */
            NumericStrengthAssignment.prototype.compare = function (cid1, cid2) {
                if (cid1 in this.strengths) {
                    if (cid2 in this.strengths) {
                        return this.strengths[cid1] - this.strengths[cid2];
                    }
                    else {
                        return -1;
                    }
                }
                else {
                    if (cid2 in this.strengths) {
                        return 1;
                    }
                    else {
                        return 0;
                    }
                }
            };
            return NumericStrengthAssignment;
        }());
        plan.NumericStrengthAssignment = NumericStrengthAssignment;
        /*==================================================================
         * This strength assignment operates by keeping an ordered list of
         * all constraints.  It is slow at updating and at comparing two
         * constraints (since it requires a linear search of the list), but
         * fast at generating an ordered list.
         */
        var ListStrengthAssignment = /** @class */ (function () {
            function ListStrengthAssignment() {
                // List of constraints from weakest to strongest
                this.strengths = [];
            }
            /*----------------------------------------------------------------
             * Add/re-order individual constraints
             */
            ListStrengthAssignment.prototype.setToMax = function (cid) {
                var i = this.strengths.indexOf(cid);
                if (i >= 0) {
                    this.strengths.splice(i, 1);
                }
                this.strengths.push(cid);
            };
            ListStrengthAssignment.prototype.setToMin = function (cid) {
                var i = this.strengths.indexOf(cid);
                if (i >= 0) {
                    this.strengths.splice(i, 1);
                }
                this.strengths.unshift(cid);
            };
            ListStrengthAssignment.prototype.setOptionals = function (cids) {
                this.strengths = cids;
            };
            /*----------------------------------------------------------------
             * Remove an individual constraint
             */
            ListStrengthAssignment.prototype.remove = function (cid) {
                var i = this.strengths.indexOf(cid);
                if (i >= 0) {
                    this.strengths.splice(i, 1);
                }
            };
            /*----------------------------------------------------------------
             * Get all optional constraints in order from weakest to
             * strongest.
             */
            ListStrengthAssignment.prototype.getList = function () {
                return this.strengths;
            };
            /*----------------------------------------------------------------
             * Comparison operator
             */
            ListStrengthAssignment.prototype.compare = function (cid1, cid2) {
                var i1 = this.strengths.indexOf(cid1);
                var i2 = this.strengths.indexOf(cid2);
                if (i1 >= 0) {
                    if (i2 >= 0) {
                        return i1 - i2;
                    }
                    else {
                        return -1;
                    }
                }
                else {
                    if (i2 >= 0) {
                        return 1;
                    }
                    else {
                        return 0;
                    }
                }
            };
            return ListStrengthAssignment;
        }());
        plan.ListStrengthAssignment = ListStrengthAssignment;
    })(plan = hd.plan || (hd.plan = {}));
})(hd || (hd = {}));
/*####################################################################
 * Defines Planner interface.
 */
/*####################################################################
 * Planner using the QuickPlan algorithm.
 */
var hd;
(function (hd) {
    var plan;
    (function (plan) {
        var u = hd.utility;
        var g = hd.graph;
        /*==================================================================
         * The planner object.
         */
        var QuickPlanner = /** @class */ (function () {
            /*----------------------------------------------------------------
             * Initialize from constraint graph.  Set all constraints to
             * be enforced.
             */
            function QuickPlanner(cgraph) {
                // The strength assignment
                this.strengths = new plan.NumericStrengthAssignment();
                this.cgraph = cgraph;
            }
            /*----------------------------------------------------------------
             * Get list of optional constraints only in order from weakest
             * to strongest.
             */
            QuickPlanner.prototype.getOptionals = function () {
                var cids = this.strengths.getOptionalsUnordered();
                cids.sort(this.strengths.compare.bind(this.strengths));
                return cids;
            };
            /*----------------------------------------------------------------
             * Reset all constraint strengths according to provided order from
             * weakest to strongest.  Any constraints not in the list are
             * assumed to be required.
             */
            QuickPlanner.prototype.setOptionals = function (order) {
                this.strengths = new plan.NumericStrengthAssignment();
                order.forEach(this.strengths.setToMax, this.strengths);
            };
            /*----------------------------------------------------------------
             * Remove single constraint from consideration.
             */
            QuickPlanner.prototype.removeOptional = function (cid) {
                this.strengths.remove(cid);
            };
            /*----------------------------------------------------------------
             * Make constraint optional (if it isn't already) and give it the
             * highest strength of all optional constraints.
             */
            QuickPlanner.prototype.setMaxStrength = function (cid) {
                this.strengths.setToMax(cid);
            };
            /*----------------------------------------------------------------
             * Make constraint optional (if it isn't already) and give it the
             * lowest strength of all optional constraints.
             */
            QuickPlanner.prototype.setMinStrength = function (cid) {
                this.strengths.setToMin(cid);
                // TODO: What needs to be resolved?
            };
            /*----------------------------------------------------------------
             */
            QuickPlanner.prototype.compare = function (cid1, cid2) {
                return this.strengths.compare(cid1, cid2);
            };
            /*----------------------------------------------------------------
             * Run planning algorithm; return true if planning succeeded.
             * Updates internal solution graph.
             */
            QuickPlanner.prototype.plan = function (oldSGraph, cidsToEnforce) {
                console.time("planner.plan");
                // Build a new QP solution graph that equals cgraph intersect sgraph
                var qpsgraph = this.sgraph = new QPSolutionGraph();
                this.cgraph.variables().forEach(qpsgraph.addVariable, qpsgraph);
                if (oldSGraph) {
                    this.cgraph.constraints().forEach(function (cid) {
                        var mid = oldSGraph.selectedForConstraint(cid);
                        if (mid) {
                            qpsgraph.addMethod(mid, cid, this.cgraph.inputsForMethod(mid), this.cgraph.outputsForMethod(mid));
                        }
                    }, this);
                }
                var qp = new QuickPlan(this.cgraph, this.strengths, qpsgraph, cidsToEnforce);
                var result = qp.run();
                console.timeEnd("planner.plan");
                return result;
            };
            /*----------------------------------------------------------------
             * Get solution graph from last successful plan.
             */
            QuickPlanner.prototype.getSGraph = function () {
                return this.sgraph;
            };
            return QuickPlanner;
        }());
        plan.QuickPlanner = QuickPlanner;
        var QPSolutionGraph = /** @class */ (function (_super) {
            __extends(QPSolutionGraph, _super);
            function QPSolutionGraph() {
                var _this = _super !== null && _super.apply(this, arguments) || this;
                // changes since last commit
                _this.changes = {};
                return _this;
            }
            /*----------------------------------------------------------------
             * Make change to the graph
             */
            QPSolutionGraph.prototype.selectMethod = function (cid, mid, inputs, outputs) {
                var oldmid = this.selectedForConstraint(cid);
                if (oldmid != mid) {
                    // record the change
                    if (cid in this.changes) {
                        if (this.changes[cid].mid == mid) {
                            delete this.changes[cid];
                        }
                    }
                    else {
                        this.changes[cid] = { mid: oldmid,
                            inputs: oldmid ? this.inputsForMethod(oldmid) : null,
                            outputs: oldmid ? this.outputsForMethod(oldmid) : null
                        };
                    }
                    // make the switch
                    _super.prototype.selectMethod.call(this, cid, mid, inputs, outputs);
                }
            };
            /*----------------------------------------------------------------
             * Rollback any changes since the last commit.
             */
            QPSolutionGraph.prototype.rollback = function () {
                for (var cid in this.changes) {
                    _super.prototype.selectMethod.call(this, cid, this.changes[cid].mid, this.changes[cid].inputs, this.changes[cid].outputs);
                }
                this.changes = {};
            };
            /*----------------------------------------------------------------
             * Commit all changes since the last commit
             * (by forgetting about them).
             */
            QPSolutionGraph.prototype.commit = function () {
                this.changes = {};
            };
            return QPSolutionGraph;
        }(g.CachingSolutionGraph));
        plan.QPSolutionGraph = QPSolutionGraph;
        /*==================================================================
         * Performs one part of the Quick Plan algorithm:  enforce a
         * single constraint by retracting weaker constraints until it can
         * be enforced.
         */
        var QPSingle = /** @class */ (function () {
            /*----------------------------------------------------------------
             * Initialize members.
             */
            function QPSingle(cidToEnforce, fullcgraph, strengths, sgraph) {
                // Strongest constraint which has been retracted so far
                this.strongestRetractedCid = null;
                // Any variables which we have made undetermined (not output by any method)
                // by modifying constraints
                this.undeterminedVids = {};
                this.timeSpentUpstreamConstraints = 0;
                this.upstreamLength = 0;
                this.runTime = 0;
                this.cidToEnforce = cidToEnforce;
                this.strengths = strengths;
                this.sgraph = sgraph;
                var now = performance.now();
                // Copy the subgraph we will be considering
                var upstreamConstraints = new g.DigraphWalker(sgraph.graph)
                    .nodesUpstreamOtherType(fullcgraph.variablesForConstraint(cidToEnforce))
                    .map(fullcgraph.constraintForMethod, fullcgraph);
                this.timeSpentUpstreamConstraints += performance.now() - now;
                this.upstreamLength = upstreamConstraints.length;
                var copyFromFull = copyConstraintsFrom(fullcgraph);
                this.subcgraph = new g.CachingConstraintGraph();
                this.subcgraph = upstreamConstraints.reduce(copyFromFull, this.subcgraph);
                this.subcgraph = copyFromFull(this.subcgraph, cidToEnforce);
                // Find retractable constraints
                this.retractableCids = new u.Heap(function (cid1, cid2) {
                    return strengths.compare(cid1, cid2) < 0;
                });
                this.retractableCids.pushAll(this.subcgraph.constraints().filter(this.isRetractable, this));
                // Initialize free variable queue
                this.freeVarQueue =
                    this.subcgraph.variables().filter(this.isFreeVar, this);
            }
            /*----------------------------------------------------------------
             * Check to see if planning has succeeded yet.
             */
            QPSingle.prototype.targetConstraintSatisfied = function () {
                return this.sgraph.selectedForConstraint(this.cidToEnforce) !== null;
            };
            /*----------------------------------------------------------------
             * Geter for strongest retracted constraint
             */
            QPSingle.prototype.getStrongestRetractedCid = function () {
                return this.strongestRetractedCid;
            };
            /*----------------------------------------------------------------
             * Getter for undetermined variables
             */
            QPSingle.prototype.getUndeterminedVids = function () {
                return u.stringSet.members(this.undeterminedVids);
            };
            /*----------------------------------------------------------------
             * Attempt to enforce target constraint by repeatedly enforcing
             * constraints for any free variables, then retracting weaker
             * constraints (hopefully freeing more variables).
             */
            QPSingle.prototype.run = function () {
                var now = performance.now();
                this.enforceConstraintsForAnyFreeVariables();
                this.runTime += performance.now() - now;
                // Keep retracting as long as their are weaker constraints
                while (!this.targetConstraintSatisfied() &&
                    this.retractableCids.length > 0) {
                    // Retract
                    var retractCid = this.retractableCids.pop();
                    this.determineConstraint(retractCid, null);
                    this.enforceConstraintsForAnyFreeVariables();
                }
                // Did it work?
                if (this.targetConstraintSatisfied()) {
                    this.sgraph.commit();
                    return true;
                }
                else {
                    this.sgraph.rollback();
                    return false;
                }
            };
            /*----------------------------------------------------------------
             * Attempt to enforce constraints for any free variables by
             * selecting the method that outputs to it.
             */
            QPSingle.prototype.enforceConstraintsForAnyFreeVariables = function () {
                while (!this.targetConstraintSatisfied() &&
                    this.freeVarQueue.length > 0) {
                    var vid = this.freeVarQueue.pop();
                    var cids = this.subcgraph.constraintsWhichOutput(vid);
                    if (cids.length == 1) { // note: length could be 0
                        var cid = cids[0];
                        var mid = this.bestSelectableMethod(cid);
                        if (mid) {
                            this.determineConstraint(cid, mid);
                        }
                    }
                }
            };
            /*----------------------------------------------------------------
             * Return the method from the specified constraint which only
             * outputs to free variables.  If more than one is found,
             * returns the one with the fewest outputs.
             */
            QPSingle.prototype.bestSelectableMethod = function (cid) {
                var mids = this.subcgraph.methodsForConstraint(cid);
                var best = null;
                var bestCount = 0;
                mids.forEach(function (mid) {
                    var outputs = this.subcgraph.outputsForMethod(mid);
                    if (outputs.every(this.isFreeVar, this)) {
                        if (best === null || bestCount > outputs.length) {
                            best = mid;
                            bestCount = outputs.length;
                        }
                    }
                }, this);
                return best;
            };
            /*----------------------------------------------------------------
             * Set constraint in the solution graph -- either to a selected
             * method, or to null if it's being retracted
             */
            QPSingle.prototype.determineConstraint = function (cid, mid) {
                // Outputs of the old method become undetermined
                var oldmid = this.sgraph.selectedForConstraint(cid);
                if (oldmid) {
                    this.sgraph.outputsForMethod(oldmid)
                        .forEach(this.makeUndetermined, this);
                }
                // Make the switch
                this.sgraph.selectMethod(cid, mid, this.subcgraph.inputsForMethod(mid), this.subcgraph.outputsForMethod(mid));
                // Outputs of the new method are no longer undetermined
                if (mid) {
                    this.sgraph.outputsForMethod(mid)
                        .forEach(this.makeDetermined, this);
                }
                // Remove constraint from constraint graph
                var outputs = this.subcgraph.outputsForConstraint(cid);
                this.subcgraph.methodsForConstraint(cid)
                    .forEach(this.subcgraph.removeMethod, this.subcgraph);
                this.freeVarQueue =
                    this.freeVarQueue.concat(outputs.filter(this.isFreeVar, this));
                // Remember the strongest retracted
                if (mid === null &&
                    (this.strongestRetractedCid === null ||
                        this.strengths.compare(this.strongestRetractedCid, cid) < 0)) {
                    this.strongestRetractedCid = cid;
                }
            };
            /*----------------------------------------------------------------
             * Simple predicate to see if a variable is free.
             */
            QPSingle.prototype.isFreeVar = function (vid) {
                var cids = this.subcgraph.constraintsWhichOutput(vid);
                return cids.length == 1;
            };
            /*----------------------------------------------------------------
             * Simple predicate to see if a constraint is retractable.
             */
            QPSingle.prototype.isRetractable = function (cid) {
                return this.strengths.compare(cid, this.cidToEnforce) < 0;
            };
            /*----------------------------------------------------------------
             * Helper
             */
            QPSingle.prototype.makeUndetermined = function (vid) {
                u.stringSet.add(this.undeterminedVids, vid);
            };
            /*----------------------------------------------------------------
             * Helper
             */
            QPSingle.prototype.makeDetermined = function (vid) {
                u.stringSet.remove(this.undeterminedVids, vid);
            };
            return QPSingle;
        }());
        /*------------------------------------------------------------------
         * Helper function for copying constraints from one constraint
         * graph to another.
         *
         * Written in curried form to facilitate applying with forEach.
         */
        function copyConstraintsFrom(source) {
            return function (destination, cid) {
                source.variablesForConstraint(cid).forEach(function (vid) {
                    if (!destination.contains(vid)) {
                        destination.addVariable(vid);
                    }
                });
                source.methodsForConstraint(cid).forEach(function (mid) {
                    destination.addMethod(mid, cid, source.inputsForMethod(mid), source.outputsForMethod(mid));
                });
                return destination;
            };
        }
        /*==================================================================
         * The full Quick Plan algorithm.
         */
        var QuickPlan = /** @class */ (function () {
            /*----------------------------------------------------------------
             * Initialize
             */
            function QuickPlan(cgraph, strengths, sgraph, cidsToEnforce) {
                this.cgraph = cgraph;
                this.sgraph = sgraph;
                this.strengths = strengths;
                // Move the cids into a max-heap
                this.cidsToEnforce = new u.Heap(function (cid1, cid2) {
                    return strengths.compare(cid1, cid2) > 0;
                });
                this.cidsToEnforce.pushAll(cidsToEnforce);
            }
            /*----------------------------------------------------------------
             * Perform the quick-plan algorithm to enforce all specified
             * constraints (if possible)
             */
            QuickPlan.prototype.run = function () {
                var allSucceeded = true;
                console.log("-------------");
                console.time("planner.run");
                var total_single = 0;
                var timeConstructor = 0;
                var single_calls = 0;
                var total_upstream_time = 0;
                var total_upstream_length = 0;
                var runTime = 0;
                // Go through constraints from strongest to weakest
                while (this.cidsToEnforce.length > 0) {
                    var cid = this.cidsToEnforce.pop();
                    single_calls += 1;
                    // Try to enforce a single constraint
                    var now = performance.now();
                    var plan1 = new QPSingle(cid, this.cgraph, this.strengths, this.sgraph);
                    timeConstructor += performance.now() - now;
                    now = performance.now();
                    var runResult = plan1.run();
                    total_single += performance.now() - now;
                    total_upstream_time += plan1.timeSpentUpstreamConstraints;
                    total_upstream_length += plan1.upstreamLength;
                    runTime += plan1.runTime;
                    if (runResult) {
                        // Add back any constraints which may have become eligible
                        var strongestRetracted = plan1.getStrongestRetractedCid();
                        if (strongestRetracted) {
                            var unenforced = this.collectDownstreamUnenforced(cid, strongestRetracted, plan1.getUndeterminedVids());
                            unenforced.forEach(function (cid) {
                                if (!this.cidsToEnforce.contains(cid)) {
                                    this.cidsToEnforce.push(cid);
                                }
                            }, this);
                        }
                    }
                    else if (this.strengths.isRequired(cid)) {
                        allSucceeded = false;
                    }
                }
                console.log("Number of QPSingle calls:", single_calls);
                console.log("  QPSingle.constructor", timeConstructor);
                console.log("    Time spent collecting upstream constraints", total_upstream_time);
                console.log("    Average number of upstream constraints", total_upstream_length / single_calls);
                console.log("  QPSingle.run", total_single);
                console.log("    Total for enforceConstraintsForAnyFreeVariables", runTime);
                console.timeEnd("planner.run");
                return allSucceeded;
            };
            /*----------------------------------------------------------------
             * If we had to retract anything then we need to check for
             * constraints which may now be eligible for enforcing.  They are
             * the ones which are (1) downstream of something we retracted,
             * and (2) weaker than what we retracted
             */
            QuickPlan.prototype.collectDownstreamUnenforced = function (cid, strongestRetracted, undetermined) {
                // First, find all the downstream variables
                var selected = this.sgraph.selectedForConstraint(cid);
                var startingVars = u.arraySet.union(this.sgraph.outputsForMethod(selected), undetermined);
                var downstreamVariables = new g.DigraphWalker(this.sgraph.graph)
                    .nodesDownstreamSameType(startingVars);
                // Get constraints which write to those variables
                var cids = downstreamVariables
                    .map(this.cgraph.constraintsWhichOutput, this.cgraph)
                    .reduce(function (collected, cids) {
                    return cids.reduce(u.stringSet.build, collected);
                }, {});
                // Return the ones which are unenforced and weaker
                return u.stringSet.members(cids)
                    .filter(function (cid) {
                    return (this.strengths.compare(cid, strongestRetracted) < 0 &&
                        this.sgraph.selectedForConstraint(cid) === null);
                }, this);
            };
            return QuickPlan;
        }());
    })(plan = hd.plan || (hd.plan = {}));
})(hd || (hd = {}));
/*####################################################################
 * Planner using compiled DFA function.
 */
var hd;
(function (hd) {
    var plan;
    (function (plan) {
        var g = hd.graph;
        /*==================================================================
         * The planner object.
         */
        var DfaFnPlanner = /** @class */ (function () {
            /*----------------------------------------------------------------
             * Initialize.
             */
            function DfaFnPlanner(cidIndexes, mids, fn, cgraph) {
                // Map stay constraints to an index number
                this.cidIndexes = {};
                this.cidCount = 0;
                // The strength assignment
                this.strengths = new plan.ListStrengthAssignment();
                this.cidIndexes = cidIndexes;
                this.cidCount = Object.keys(cidIndexes).length;
                this.mids = mids;
                this.fn = fn;
                this.cgraph = cgraph;
            }
            /*----------------------------------------------------------------
             * Get list of optional constraints only in order from weakest
             * to strongest.
             */
            DfaFnPlanner.prototype.getOptionals = function () {
                var reverseMap = [];
                for (var cid in this.cidIndexes) {
                    reverseMap[this.cidIndexes[cid]] = cid;
                }
                return this.strengths.getList().map(function (idx) {
                    return reverseMap[idx];
                });
            };
            /*----------------------------------------------------------------
             * Reset all constraint strengths according to provided order from
             * weakest to strongest.  Any constraints not in the list are
             * assumed to be required.
             */
            DfaFnPlanner.prototype.setOptionals = function (order) {
                for (var i = 0, l = order.length; i < l; ++i) {
                    var cid = order[i];
                    if (!(cid in this.cidIndexes)) {
                        this.cidIndexes[cid] = this.cidCount++;
                    }
                }
                this.strengths.setOptionals(order.map(function (cid) {
                    return this.cidIndexes[cid];
                }, this));
            };
            /*----------------------------------------------------------------
             * Remove single constraint from consideration.
             */
            DfaFnPlanner.prototype.removeOptional = function (cid) {
                console.error("Cannot make modifications to constraint graph with DFA planner");
            };
            /*----------------------------------------------------------------
             * Make constraint optional (if it isn't already) and give it the
             * highest strength of all optional constraints.
             */
            DfaFnPlanner.prototype.setMaxStrength = function (cid) {
                var idx = this.cidIndexes[cid];
                if (idx === undefined) {
                    idx = this.cidIndexes[cid] = this.cidCount++;
                }
                this.strengths.setToMax(idx);
            };
            /*----------------------------------------------------------------
             * Make constraint optional (if it isn't already) and give it the
             * lowest strength of all optional constraints.
             */
            DfaFnPlanner.prototype.setMinStrength = function (cid) {
                var idx = this.cidIndexes[cid];
                if (idx === undefined) {
                    idx = this.cidIndexes[cid] = this.cidCount++;
                }
                this.strengths.setToMin(idx);
            };
            /*----------------------------------------------------------------
             * Test whether first is stronger than second.
             */
            DfaFnPlanner.prototype.compare = function (cid1, cid2) {
                return this.strengths.compare(this.cidIndexes[cid1], this.cidIndexes[cid2]);
            };
            /*----------------------------------------------------------------
             * Run planning algorithm; return true if planning succeeded.
             */
            DfaFnPlanner.prototype.plan = function () {
                var selectedMethods = this.fn.call(null, this.strengths.getList());
                if (selectedMethods) {
                    this.selectedMethods = selectedMethods;
                    return true;
                }
                else {
                    return false;
                }
            };
            /*----------------------------------------------------------------
             * Get solution graph from last successful plan.
             */
            DfaFnPlanner.prototype.getSGraph = function () {
                var sgraph = new g.CachingSolutionGraph();
                this.cgraph.variables().forEach(sgraph.addVariable, sgraph);
                for (var i = 0, l = this.selectedMethods.length; i < l; ++i) {
                    var mid = this.mids[this.selectedMethods[i]];
                    sgraph.addMethod(mid, this.cgraph.constraintForMethod(mid), this.cgraph.inputsForMethod(mid), this.cgraph.outputsForMethod(mid));
                }
                return sgraph;
            };
            return DfaFnPlanner;
        }());
        plan.DfaFnPlanner = DfaFnPlanner;
    })(plan = hd.plan || (hd.plan = {}));
})(hd || (hd = {}));
/*####################################################################
 * Planner which creates and simulates a DFA.
 */
var hd;
(function (hd) {
    var plan;
    (function (plan) {
        var g = hd.graph;
        var d = hd.dfa;
        /*==================================================================
         * The planner object
         */
        var ComposedPlanner = /** @class */ (function () {
            /*----------------------------------------------------------------
             * Initialize.
             */
            function ComposedPlanner(cgraph) {
                // The graph of the single, composed constraint
                this.compcgraph = new g.CachingConstraintGraph();
                // The strength assignment
                this.strengths = new plan.ListStrengthAssignment();
                this.cgraph = cgraph;
                this.recompose();
            }
            /*----------------------------------------------------------------
             * Get list of optional constraints only in order from weakest
             * to strongest.
             */
            ComposedPlanner.prototype.getOptionals = function () {
                return this.strengths.getList();
            };
            /*----------------------------------------------------------------
             * Reset all constraint strengths according to provided order from
             * weakest to strongest.  Any constraints not in the list are
             * assumed to be required.
             */
            ComposedPlanner.prototype.setOptionals = function (order) {
                this.strengths.setOptionals(order);
            };
            /*----------------------------------------------------------------
             * Remove single constraint from consideration.
             */
            ComposedPlanner.prototype.removeOptional = function (cid) {
                this.strengths.remove(cid);
            };
            /*----------------------------------------------------------------
             * Make constraint optional (if it isn't already) and give it the
             * highest strength of all optional constraints.
             */
            ComposedPlanner.prototype.setMaxStrength = function (cid) {
                this.strengths.setToMax(cid);
            };
            /*----------------------------------------------------------------
             * Make constraint optional (if it isn't already) and give it the
             * lowest strength of all optional constraints.
             */
            ComposedPlanner.prototype.setMinStrength = function (cid) {
                this.strengths.setToMin(cid);
            };
            /*----------------------------------------------------------------
             * Test whether first is stronger than second.
             */
            ComposedPlanner.prototype.compare = function (cid1, cid2) {
                return this.strengths.compare(cid1, cid2);
            };
            /*----------------------------------------------------------------
             * Run planning algorithm; return true if planning succeeded.
             */
            ComposedPlanner.prototype.plan = function (sgraph, cidsToEnforce) {
                // If changes have been made to cgraph must rebuild DFA
                if (cidsToEnforce.some(g.isNotStayConstraint)) {
                    this.recompose();
                }
                // Simulate DFA
                this.selected = d.runDfa(this.dfa, this.strengths.getList());
                return this.selected ? true : false;
            };
            /*----------------------------------------------------------------
             * Get solution graph from last successful plan.
             */
            ComposedPlanner.prototype.getSGraph = function () {
                var sgraph = new g.CachingSolutionGraph();
                this.cgraph.variables().forEach(sgraph.addVariable, sgraph);
                var selectedMethods = this.composite.compmids[this.selected];
                for (var i = 0, l = selectedMethods.length; i < l; ++i) {
                    var mid = selectedMethods[i];
                    sgraph.addMethod(mid, this.cgraph.constraintForMethod(mid), this.cgraph.inputsForMethod(mid), this.cgraph.outputsForMethod(mid));
                }
                return sgraph;
            };
            /*----------------------------------------------------------------
             */
            ComposedPlanner.prototype.recompose = function () {
                this.composite = d.composeAllConstraints(this.cgraph);
                this.compcgraph.methods().forEach(this.compcgraph.removeMethod, this.compcgraph);
                this.compcgraph.variables().forEach(this.compcgraph.removeVariable, this.compcgraph);
                this.cgraph.variables().forEach(this.compcgraph.addVariable, this.compcgraph);
                d.addCompositeMethods(this.compcgraph, this.cgraph, this.composite);
                this.dfa = new d.SoftLinkedDfa();
                d.compileToDfa(this.dfa, this.compcgraph, d.Order.High);
            };
            return ComposedPlanner;
        }());
        plan.ComposedPlanner = ComposedPlanner;
    })(plan = hd.plan || (hd.plan = {}));
})(hd || (hd = {}));
//
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (b.hasOwnProperty(p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
/*####################################################################
 * The EnablementLabels class.
 */
var hd;
(function (hd) {
    var enable;
    (function (enable) {
        var u = hd.utility;
        var r = hd.reactive;
        /*==================================================================
         * Label for an edge in the enablement graph.
         */
        var Label;
        (function (Label) {
            Label[Label["Irrelevant"] = 1] = "Irrelevant";
            Label[Label["AssumedRelevant"] = 2] = "AssumedRelevant";
            Label[Label["Relevant"] = 3] = "Relevant";
        })(Label = enable.Label || (enable.Label = {}));
        ;
        /*==================================================================
         * The enablement graph is the same as the solution graph with
         * labels.  So, rather than copying the structure of the solution
         * graph, we just store here the labels.  Thus,
         *   SolutionGraph + EnablementLabels = EnablementGraph
         */
        var EnablementLabels = /** @class */ (function (_super) {
            __extends(EnablementLabels, _super);
            function EnablementLabels() {
                var _this = _super !== null && _super.apply(this, arguments) || this;
                // (cid -> mid)
                _this.selected = {};
                // Label for each method  (mid -> vid -> label)
                _this.labels = {};
                _this.scheduled = false;
                return _this;
            }
            /*----------------------------------------------------------------
             * Indicate new selected method for constraint.
             * Primarily just deletes any existing labels for that constraint.
             */
            EnablementLabels.prototype.selectMethod = function (cid, mid) {
                var oldmid = this.selected[cid];
                var labeled = false;
                if (oldmid) {
                    var oldmidLabels = this.labels[oldmid];
                    // Check for any non-default labels for oldmid
                    for (var vid in oldmidLabels) {
                        if (oldmidLabels[vid] !== Label.AssumedRelevant) {
                            labeled = true;
                            break;
                        }
                    }
                    // Git rid of oldmid labels
                    if (mid == oldmid) {
                        if (labeled) {
                            this.labels[oldmid] = {};
                        }
                    }
                    else {
                        delete this.labels[oldmid];
                    }
                }
                // Initialize mid labels
                if (mid && mid != oldmid) {
                    this.labels[mid] = {};
                }
                // Report only if method or labels changed
                this.selected[cid] = mid;
                if (oldmid !== mid || labeled) {
                    this.schedule();
                }
            };
            /*----------------------------------------------------------------
             * Get label for edge (vid, mid).  Does not check to see if
             * (vid, mid) is actually in the graph; default return value is
             * AssumedRelevant.
             */
            EnablementLabels.prototype.getLabel = function (vid, mid) {
                var midLabels = this.labels[mid];
                if (midLabels) {
                    var label = midLabels[vid];
                    return label ? label : Label.AssumedRelevant;
                }
                else {
                    return Label.AssumedRelevant;
                }
            };
            /*----------------------------------------------------------------
             * Set label for edge (vid, mid).  Has no effect if mid is not
             * selected method.
             */
            EnablementLabels.prototype.setLabel = function (vid, mid, label) {
                var midLabels = this.labels[mid];
                if (midLabels) {
                    if (midLabels[vid] !== label) {
                        midLabels[vid] = label;
                        this.schedule();
                    }
                }
            };
            /*----------------------------------------------------------------
             * Schedule a call to notify (unless one is already scheduled)
             */
            EnablementLabels.prototype.schedule = function () {
                if (!this.scheduled) {
                    this.scheduled = true;
                    u.schedule(4, this.notify, this);
                }
            };
            /*----------------------------------------------------------------
             * Perform an notify
             */
            EnablementLabels.prototype.notify = function () {
                this.scheduled = false;
                this.sendNext(this);
            };
            return EnablementLabels;
        }(r.BasicObservable));
        enable.EnablementLabels = EnablementLabels;
    })(enable = hd.enable || (hd.enable = {}));
})(hd || (hd = {}));
/*####################################################################
 * The UsageReporter class.
 */
var hd;
(function (hd) {
    var enable;
    (function (enable) {
        var r = hd.reactive;
        /*==================================================================
         * Updates enablement graph labels to reflect whether inputs were
         * used in a specific method invocation.
         *
         * Uses "Assumed" labels until all output promises have been
         * resolved; then switches to regular labels.
         *
         * Does not keep pointers to any promises; simply subscribes to
         * them.  Thus, when promises are reclaimed, these will be too.
         */
        var EnableReporter = /** @class */ (function () {
            /*----------------------------------------------------------------
             * Initialize and subscribe
             */
            function EnableReporter(egraph, mid, inputs, outputs) {
                // How many output promises have completed
                this.completedCount = 0;
                this.egraph = egraph;
                this.mid = mid;
                this.inputVids = Object.keys(inputs);
                this.outputCount = 0;
                // Watch dependency counts of all inputs
                for (var vid in inputs) {
                    inputs[vid].usage.addObserver(this, this.onNextUsage, null, null, vid);
                }
                // Watch all outputs
                for (var vid in outputs) {
                    outputs[vid].addObserver(this, null, null, this.onCompletedOutput);
                    ++this.outputCount;
                }
            }
            /*----------------------------------------------------------------
             * When an output promise completes
             */
            EnableReporter.prototype.onCompletedOutput = function () {
                if (++this.completedCount == this.outputCount && this.egraph) {
                    for (var i = 0, l = this.inputVids.length; i < l; ++i) {
                        var vid = this.inputVids[i];
                        if (this.egraph.getLabel(vid, this.mid) === enable.Label.AssumedRelevant) {
                            this.egraph.setLabel(vid, this.mid, enable.Label.Irrelevant);
                        }
                    }
                }
            };
            /*----------------------------------------------------------------
             * When input usage changes
             */
            EnableReporter.prototype.onNextUsage = function (usage, vid) {
                if (this.egraph) {
                    if (usage === r.Usage.Used) {
                        this.egraph.setLabel(vid, this.mid, enable.Label.Relevant);
                    }
                    else if (usage === r.Usage.Unused) {
                        this.egraph.setLabel(vid, this.mid, enable.Label.Irrelevant);
                    }
                    else if (this.completedCount < this.outputCount) {
                        this.egraph.setLabel(vid, this.mid, enable.Label.AssumedRelevant);
                    }
                }
            };
            /*----------------------------------------------------------------
             * Stop reporting results to enablement graph
             */
            EnableReporter.prototype.cancel = function () {
                this.egraph = null;
            };
            return EnableReporter;
        }());
        enable.EnableReporter = EnableReporter;
        /*==================================================================
         * Enablement manager is used to keep track of all the
         * EnableReporters and make sure that they get canceled when their
         * results are overwritten.  Also manages changes to egraph.
         */
        var EnablementManager = /** @class */ (function () {
            function EnablementManager() {
                // The enablement graph
                this.egraph = new enable.EnablementLabels();
                // The reporters
                this.reporters = {};
            }
            /*----------------------------------------------------------------
             * Used to indicate a new method has been scheduled.  Updates
             * the enablement graph, creates new reporters, cancels old ones.
             */
            EnablementManager.prototype.methodScheduled = function (cid, mid, inputs, outputs) {
                this.egraph.selectMethod(cid, mid);
                if (this.reporters[cid]) {
                    this.reporters[cid].cancel();
                    if (!mid) {
                        delete this.reporters[cid];
                    }
                }
                if (mid) {
                    this.reporters[cid] =
                        new EnableReporter(this.egraph, mid, inputs, outputs);
                }
            };
            return EnablementManager;
        }());
        enable.EnablementManager = EnablementManager;
    })(enable = hd.enable || (hd.enable = {}));
})(hd || (hd = {}));
/*####################################################################
 * Enablement checking functions
 */
var hd;
(function (hd) {
    var enable;
    (function (enable) {
        var u = hd.utility;
        var g = hd.graph;
        /*==================================================================
         * Check all variables for contributing.  A variable is contributing
         * if it can reach an output variable in the current solution graph.
         * We can check all variables at once by searching backwards
         * starting with the output variables.
         *
         * Returns dictionary mapping variable id to an enablement label.
         * For a given vid, the label will be:
         *   if there is a path to an output containing only
         *          Relevant edges
         *     => Relevant
         *   else if there is a path to an output containing only
         *          Relevant and AssumedRelevant edges
         *     => AssumedRelevant
         *   else if there is a path to an output
         *     => Irrelevant
         *   else
         *     => undefined
         *
         */
        function globalContributingCheck(sgraph, egraph, outputVids) {
            var contributing = {};
            // Perform reverse-search for each output
            outputVids.forEach(function (vid) {
                flood(vid, enable.Label.Relevant);
            });
            return contributing;
            /*----------------------------------------------------------------
             * Recursive searching routine.  Note that we may wind up
             * exploring a node multiple times if it has the potential to
             * improve a variables label.
             */
            function flood(vid, label) {
                if (label > (contributing[vid] || 0)) {
                    contributing[vid] = label;
                    // Travel up to any methods which output this variable
                    var nextMids = sgraph.methodsWhichOutput(vid);
                    for (var i = 0, l = nextMids.length; i < l; ++i) {
                        var nextMid = nextMids[i];
                        // Travel up to any variables which this method inputs
                        var nextVids = sgraph.inputsForMethod(nextMid);
                        nextVids.sort(function (a, b) {
                            return egraph.getLabel(b, nextMid) - egraph.getLabel(a, nextMid);
                        });
                        for (var j = 0, m = nextVids.length; j < m; ++j) {
                            var nextVid = nextVids[j];
                            // Recurse
                            var nextLabel = Math.min(label, egraph.getLabel(nextVid, nextMid));
                            flood(nextVid, nextLabel);
                        }
                    }
                }
            }
        }
        enable.globalContributingCheck = globalContributingCheck;
        /*==================================================================
         * Check a single variable for relevancy.  It's relevant if there
         * is some plan containing a path from the variable to the output
         * that does not contain any edges labeled Irrelevant in egraph.
         *
         * Note that this test can only return Yes or No.
         */
        function relevancyCheck(cgraph, egraph, outputVidSet, vid) {
            // Map of cid -> vid
            var selected = {};
            // Also used to trace our steps so that we do not select two
            //   methods from the same constraint
            return search(vid) ? u.Fuzzy.Yes : u.Fuzzy.No;
            /*----------------------------------------------------------------
             * Recursive searching function.  We search by building a path
             * from the variable method by method until we reach an output
             * variable.  Then we make sure that we can still find a plan
             * if we enforce the methods we've followed.
             */
            function search(vid) {
                if (outputVidSet[vid] && hasPlan(cgraph, selected)) {
                    return true;
                }
                var nextMids = cgraph.methodsWhichInput(vid);
                for (var i = 0, l = nextMids.length; i < l; ++i) {
                    var nextMid = nextMids[i];
                    var nextCid = cgraph.constraintForMethod(nextMid);
                    if (!selected[nextCid] &&
                        egraph.getLabel(vid, nextMid) !== enable.Label.Irrelevant) {
                        selected[nextCid] = nextMid;
                        var nextVids = cgraph.outputsForMethod(nextMid);
                        for (var j = 0, m = nextVids.length; j < m; ++j) {
                            if (search(nextVids[j])) {
                                return true;
                            }
                        }
                        selected[nextCid] = null;
                    }
                }
                return false;
            }
        }
        enable.relevancyCheck = relevancyCheck;
        /*==================================================================
         * Given a constraint graph and a set of methods, see if there is
         * a plan which uses those methods.
         */
        function hasPlan(cgraph, selected) {
            var cidsLeft = {};
            var numCidsLeft = 0;
            var vidCounts = {};
            var freeVarQueue = [];
            initCounts();
            enforceConstraintsForAnyFreeVariables();
            return numCidsLeft == 0;
            /*----------------------------------------------------------------
             * Count up number of constraints each variable uses.  Init
             * free variable queue.
             */
            function initCounts() {
                var cids = cgraph.constraints();
                for (var i = 0, l = cids.length; i < l; ++i) {
                    var cid = cids[i];
                    if (g.isStayConstraint(cid)) {
                        continue;
                    }
                    if (selected[cid]) {
                        // Mark output variables so they are never considered free
                        var vids = cgraph.outputsForMethod(selected[cid]);
                        for (var j = 0, m = vids.length; j < m; ++j) {
                            vidCounts[vids[j]] = -1;
                        }
                    }
                    else {
                        cidsLeft[cid] = true;
                        ++numCidsLeft;
                        var vids = cgraph.outputsForConstraint(cid);
                        for (var j = 0, m = vids.length; j < m; ++j) {
                            var vid = vids[j];
                            var count = vidCounts[vid];
                            if (count != -1) {
                                vidCounts[vid] = count ? count + 1 : 1;
                            }
                        }
                    }
                }
                freeVarQueue = cgraph.variables().filter(isFreeVar);
            }
            /*----------------------------------------------------------------
             * Repeatedly "enforce" any constraints which have a method that
             * writes only to free variables.  We use "enforce" loosely to
             * mean adjust all counters so that it is no longer considered.
             */
            function enforceConstraintsForAnyFreeVariables() {
                while (numCidsLeft > 0 && freeVarQueue.length > 0) {
                    var vid = freeVarQueue.pop();
                    var cids = cgraph.constraintsWhichOutput(vid).filter(isCidLeft);
                    if (cids.length == 1) {
                        var cid = cids[0];
                        if (hasSelectableMethod(cid)) {
                            determineConstraint(cid);
                        }
                    }
                }
            }
            /*----------------------------------------------------------------
             * Check whether there is at least one method in the constraint
             * that writes to only free variables.  We don't care which one
             * it is, only that it exists.
             */
            function hasSelectableMethod(cid) {
                var mids = cgraph.methodsForConstraint(cid);
                for (var i = 0, l = mids.length; i < l; ++i) {
                    var mid = mids[i];
                    var outputs = cgraph.outputsForMethod(mid);
                    if (outputs.every(isFreeVar)) {
                        return true;
                    }
                }
                return false;
            }
            /*----------------------------------------------------------------
             * Adjust counters so that constraint no longer considered.
             */
            function determineConstraint(cid) {
                cidsLeft[cid] = false;
                --numCidsLeft;
                var outputs = cgraph.outputsForConstraint(cid)
                    .forEach(function (vid) {
                    if (--vidCounts[vid] == 1) {
                        freeVarQueue.push(vid);
                    }
                });
            }
            /*----------------------------------------------------------------
             * Helper - is variable a free variable?
             */
            function isFreeVar(vid) {
                return vidCounts[vid] == 1;
            }
            /*----------------------------------------------------------------
             * Helper - is constraint still being considered?
             */
            function isCidLeft(cid) {
                return cidsLeft[cid];
            }
        }
    })(enable = hd.enable || (hd.enable = {}));
})(hd || (hd = {}));
//
/*********************************************************************
 * A PropertyModel's duties are
 *   1. Translate a model into a constraint graph
 *   2. Watch the variables to see when they change
 *   3. Run the planner to get a new solution graph
 *   4. Run the evaluator to produce new values
 */
var hd;
(function (hd) {
    var config;
    (function (config) {
        config.defaultPlannerType = hd.plan.QuickPlanner;
        config.forwardEmergingSources = false;
    })(config = hd.config || (hd.config = {}));
})(hd || (hd = {}));
(function (hd) {
    var system;
    (function (system) {
        var u = hd.utility;
        var r = hd.reactive;
        var m = hd.model;
        var g = hd.graph;
        var e = hd.enable;
        var c = hd.config;
        // scheduling priority for responding to state changes
        system.SystemUpdatePriority = 1;
        system.SystemCommandPriority = 2;
        /*==================================================================
         * The constraint system
         */
        var PropertyModel = /** @class */ (function () {
            /*----------------------------------------------------------------
             * Initialize members
             */
            function PropertyModel(plannerT, cgraphT) {
                if (plannerT === void 0) { plannerT = c.defaultPlannerType; }
                if (cgraphT === void 0) { cgraphT = g.CachingConstraintGraph; }
                // The current solution graph
                this.sgraph = null;
                // Lookup tables - convert ids back to objects
                this.variables = {};
                this.methods = {};
                this.constraints = {};
                // Update strategy
                this.scheduleUpdateOnChange = true;
                this.isUpdateScheduled = false;
                // Flag to report when system is solved (no pending variables)
                this.solved = new r.ScheduledSignal(true);
                this.pendingCount = 0;
                // Changes requiring an update
                this.needUpdating = [];
                this.needEnforcing = {};
                this.needEvaluating = {};
                // Flage indicating when update is needed --
                //   i.e. when one of the above elements is non-empty
                this.isUpdateNeeded = false;
                // Touch dependencies
                this.touchDeps = {};
                // Enablement
                this.enable = new e.EnablementManager();
                this.outputVids = {};
                this.commandQueue = [];
                this.accumPromises = [];
                this.isCommandScheduled = false;
                this.cgraph = new cgraphT();
                if (plannerT) {
                    this.planner = new plannerT(this.cgraph);
                }
                this.enable.egraph.addObserver(this, this.onNextEgraph, null, null);
            }
            PropertyModel.prototype.getCGraph = function () { return this.cgraph; };
            PropertyModel.prototype.getSGraph = function () { return this.sgraph; };
            PropertyModel.prototype.getPlanner = function () {
                if (!this.planner) {
                    this.planner = new c.defaultPlannerType(this.cgraph);
                }
                return this.planner;
            };
            /*----------------------------------------------------------------
             * Replace existing planner with new planner.
             */
            PropertyModel.prototype.switchToNewPlanner = function (plannerT) {
                var newPlanner = new plannerT(this.cgraph);
                if (this.planner) {
                    var oldPlanner = this.planner;
                    newPlanner.setOptionals(oldPlanner.getOptionals());
                }
                this.planner = newPlanner;
                this.cgraph.constraints().forEach(this.recordConstraintChange, this);
            };
            /*----------------------------------------------------------------
             * Add and remove elements to and from the property model
             */
            PropertyModel.prototype.add = function (el) {
                if (el instanceof m.Variable) {
                    this.addVariable(el);
                }
                else if (el instanceof m.Component) {
                    this.addComponent(el);
                }
                else if (el instanceof m.Constraint) {
                    this.addConstraint(el);
                }
                else if (el instanceof m.Output) {
                    this.addOutput(el.variable);
                }
                else if (el instanceof m.TouchDep) {
                    this.addTouchDependency(el.from, el.to);
                }
                else if (el instanceof m.Command) {
                    this.addCommand(el);
                }
            };
            /*----------------------------------------------------------------
             */
            PropertyModel.prototype.remove = function (el) {
                if (el instanceof m.Variable) {
                    this.removeVariable(el);
                }
                else if (el instanceof m.Component) {
                    this.removeComponent(el);
                }
                else if (el instanceof m.Constraint) {
                    this.removeConstraint(el);
                }
                else if (el instanceof m.Output) {
                    this.removeOutput(el.variable);
                }
                else if (el instanceof m.TouchDep) {
                    this.removeTouchDependency(el.from, el.to);
                }
                else if (el instanceof m.Command) {
                    this.removeCommand(el);
                }
            };
            //--------------------------------------------
            // Add component
            PropertyModel.prototype.addComponent = function (component) {
                m.Component.update(component);
                m.Component.elements(component).forEach(this.add, this);
                m.Component.changes(component).addObserver(this, this.recordComponentChange, null, null);
            };
            //--------------------------------------------
            // Remove component
            PropertyModel.prototype.removeComponent = function (component) {
                m.Component.elements(component).forEach(this.remove, this);
                m.Component.changes(component).removeObserver(this);
                this.removeComponentRecords(component);
            };
            //--------------------------------------------
            // Add variable
            PropertyModel.prototype.addVariable = function (vv) {
                if (!(vv.id in this.variables)) {
                    this.variables[vv.id] = vv;
                    // Watch for variable events
                    if (vv.pending.get()) {
                        ++this.pendingCount;
                    }
                    vv.changes.addObserver(this, this.onNextVariableChange, null, null);
                    // Create stay constraint
                    var stayMethodId = g.stayMethod(vv.id);
                    var stayConstraintId = g.stayConstraint(vv.id);
                    // Add variable+stay to existing graphs
                    this.cgraph.addVariable(vv.id);
                    this.cgraph.addMethod(stayMethodId, stayConstraintId, [], [vv.id]);
                    // Set stay to optional
                    if (vv.optional === 1 /* Max */) {
                        this.getPlanner().setMaxStrength(stayConstraintId);
                    }
                    else if (vv.optional === 2 /* Min */) {
                        this.getPlanner().setMinStrength(stayConstraintId);
                    }
                    // Mark stay constraint as changed
                    this.recordConstraintChange(stayConstraintId);
                }
            };
            //--------------------------------------------
            // Remove variable
            PropertyModel.prototype.removeVariable = function (vv) {
                if (vv.id in this.variables) {
                    var stayConstraintId = g.stayConstraint(vv.id);
                    // Remove all references
                    delete this.variables[vv.id];
                    delete this.outputVids[vv.id];
                    this.removeConstraintRecords(stayConstraintId);
                    vv.changes.removeObserver(this);
                    // Remove from graphs
                    this.cgraph.removeMethod(g.stayMethod(vv.id));
                    this.planner.removeOptional(stayConstraintId);
                    this.cgraph.removeVariable(vv.id);
                }
            };
            //--------------------------------------------
            // Add constraint
            PropertyModel.prototype.addConstraint = function (cc) {
                if (!(cc.id in this.constraints) && cc.methods.length > 0) {
                    this.constraints[cc.id] = cc;
                    cc.methods.forEach(this.addMethod.bind(this, cc.id));
                    if (cc.optional === 1 /* Max */) {
                        this.getPlanner().setMaxStrength(cc.id);
                        this.hasOptionals = true;
                    }
                    else if (cc.optional === 2 /* Min */) {
                        this.getPlanner().setMinStrength(cc.id);
                        this.hasOptionals = true;
                    }
                    if (cc.touchVariables) {
                        cc.touchVariables.forEach(function (v) {
                            this.addTouchDependency(v, cc);
                        }, this);
                    }
                    // Mark for update
                    this.recordConstraintChange(cc.id);
                }
            };
            //--------------------------------------------
            // Remove constraint
            PropertyModel.prototype.removeConstraint = function (cc) {
                if (cc.id in this.constraints) {
                    delete this.constraints[cc.id];
                    this.removeConstraintRecords(cc.id);
                    if (cc.touchVariables) {
                        cc.touchVariables.forEach(function (v) {
                            this.removeTouchDependency(v, cc);
                        }, this);
                    }
                    // For all variables being written to by this constraint:
                    //   add stay constraint to needEnforcing
                    if (this.sgraph) {
                        var mid = this.sgraph.selectedForConstraint(cc.id);
                        this.sgraph.outputsForMethod(mid).forEach(function (vid) {
                            if (this.variables[vid]) {
                                var cids = this.cgraph.constraintsWhichOutput(vid);
                                for (var i = 0, l = cids.length; i < l; ++i) {
                                    if (!this.sgraph.selectedForConstraint(cids[i])) {
                                        this.needEnforcing[cids[i]] = true;
                                    }
                                }
                            }
                        }, this);
                    }
                    cc.methods.forEach(this.removeMethod, this);
                    this.sgraph.selectMethod(cc.id, null);
                    this.enable.methodScheduled(cc.id, null);
                }
            };
            //--------------------------------------------
            // Add method
            PropertyModel.prototype.addMethod = function (cid, mm) {
                if (!(mm.id in this.methods)) {
                    this.methods[mm.id] = mm;
                    // Add to constraint graph
                    this.cgraph.addMethod(mm.id, cid, mm.inputVars.map(u.getId), mm.outputVars.map(u.getId));
                }
            };
            //--------------------------------------------
            // Remove method
            PropertyModel.prototype.removeMethod = function (mm) {
                if (mm.id in this.methods) {
                    delete this.methods[mm.id];
                    // Remove from constraint graph
                    this.cgraph.removeMethod(mm.id);
                }
            };
            //--------------------------------------------
            // Add command
            PropertyModel.prototype.addCommand = function (cmd) {
                cmd.addObserver(this, this.performCommand, null, null);
            };
            //--------------------------------------------
            // Remove command
            PropertyModel.prototype.removeCommand = function (cmd) {
                cmd.removeObserver(this);
            };
            //--------------------------------------------
            // Add touch dependency
            PropertyModel.prototype.addTouchDependency = function (cc1, cc2) {
                var cid1, cid2;
                if (cc1 instanceof m.Variable) {
                    cid1 = g.stayConstraint(cc1.id);
                }
                else {
                    cid1 = cc1.id;
                }
                if (cc2 instanceof m.Variable) {
                    cid2 = g.stayConstraint(cc2.id);
                }
                else {
                    cid2 = cc2.id;
                }
                if (this.touchDeps[cid1]) {
                    u.arraySet.add(this.touchDeps[cid1], cid2);
                }
                else {
                    this.touchDeps[cid1] = [cid2];
                }
            };
            PropertyModel.prototype.addTouchDependencies = function (cc1, cc2s) {
                for (var i = 0, l = cc2s.length; i < l; ++i) {
                    this.addTouchDependency(cc1, cc2s[i]);
                }
            };
            PropertyModel.prototype.addTouchSet = function (ccs) {
                for (var i = 0, l = ccs.length; i < l; ++i) {
                    for (var j = 0; j < l; ++j) {
                        if (i != j) {
                            this.addTouchDependency(ccs[i], ccs[j]);
                        }
                    }
                }
            };
            //--------------------------------------------
            // Remove touch dependency
            PropertyModel.prototype.removeTouchDependency = function (cc1, cc2) {
                var cid1, cid2;
                if (cc1 instanceof m.Variable) {
                    cid1 = g.stayConstraint(cc1.id);
                }
                else {
                    cid1 = cc1.id;
                }
                if (cc2 instanceof m.Variable) {
                    cid2 = g.stayConstraint(cc2.id);
                }
                else {
                    cid2 = cc2.id;
                }
                var deps = this.touchDeps[cid1];
                if (deps) {
                    u.arraySet.remove(this.touchDeps[cid1], cid2);
                }
            };
            PropertyModel.prototype.removeTouchDependencies = function (cc1, cc2s) {
                for (var i = 0, l = cc2s.length; i < l; ++i) {
                    this.removeTouchDependency(cc1, cc2s[i]);
                }
            };
            PropertyModel.prototype.removeTouchSet = function (ccs) {
                for (var i = 0, l = ccs.length; i < l; ++i) {
                    for (var j = 0; j < l; ++j) {
                        if (i != j) {
                            this.removeTouchDependency(ccs[i], ccs[j]);
                        }
                    }
                }
            };
            //--------------------------------------------
            // Add output
            PropertyModel.prototype.addOutput = function (out) {
                var id = out.id;
                if (id in this.outputVids) {
                    ++this.outputVids[id];
                }
                else {
                    this.outputVids[id] = 1;
                }
            };
            //--------------------------------------------
            // Remove output
            PropertyModel.prototype.removeOutput = function (out) {
                var id = out.id;
                if (this.outputVids[id] > 1) {
                    --this.outputVids[id];
                }
                else {
                    delete this.outputVids[out.id];
                }
            };
            /*----------------------------------------------------------------
             * Respond to variable changes
             */
            //--------------------------------------------
            // Dispatcher
            PropertyModel.prototype.onNextVariableChange = function (event) {
                switch (event.type) {
                    case 1 /* touched */:
                        this.variableTouched(event.vv);
                        break;
                    case 0 /* changed */:
                        this.variableChanged(event.vv);
                        break;
                    case 2 /* pending */:
                        ++this.pendingCount;
                        break;
                    case 3 /* settled */:
                        if (this.pendingCount > 0) {
                            --this.pendingCount;
                            if (this.pendingCount == 0 && !this.isUpdateNeeded) {
                                this.solved.set(true);
                            }
                        }
                        break;
                    case 4 /* command */:
                        this.performCommand(event.cmd);
                        break;
                }
            };
            //--------------------------------------------
            // Touch variable and all touch dependencies
            PropertyModel.prototype.doPromotions = function (originalVid) {
                var planner = this.getPlanner();
                var descending = function (cid1, cid2) {
                    return planner.compare(cid2, cid1);
                };
                var promote = [];
                var i = 0;
                var visited = {};
                promote.push(originalVid);
                visited[originalVid] = true;
                while (i < promote.length) {
                    var l = promote.length;
                    var sub = [];
                    while (i < l) {
                        var vid = promote[i++];
                        var deps = this.touchDeps[vid];
                        if (deps) {
                            deps.forEach(function (dep) {
                                if (!visited[dep] &&
                                    (!this.constraints[dep] ||
                                        this.constraints[dep].optional !== 0 /* Default */)) {
                                    sub.push(dep);
                                    visited[dep] = true;
                                }
                            }, this);
                        }
                    }
                    promote.push.apply(promote, sub.sort(descending));
                }
                for (--i; i >= 0; --i) {
                    var cid = promote[i];
                    planner.setMaxStrength(cid);
                    if (!this.sgraph ||
                        !this.sgraph.selectedForConstraint(cid)) {
                        this.recordConstraintChange(cid);
                    }
                }
            };
            //--------------------------------------------
            // Promote variable
            PropertyModel.prototype.variableTouched = function (vv) {
                var stayConstraintId = g.stayConstraint(vv.id);
                this.doPromotions(stayConstraintId);
            };
            //--------------------------------------------
            // Promote variable and mark as changed
            PropertyModel.prototype.variableChanged = function (vv) {
                var stayConstraintId = g.stayConstraint(vv.id);
                this.doPromotions(stayConstraintId);
                this.recordVariableChange(stayConstraintId);
            };
            /*----------------------------------------------------------------
             * Changes to the model are recorded for the next update.  These
             * functions add-to/remove-from those records.
             */
            //--------------------------------------------
            // Component reference has changed; need to query for adds/drops
            PropertyModel.prototype.recordComponentChange = function (cmp) {
                this.needUpdating.push(cmp);
                this.recordChange();
            };
            //--------------------------------------------
            // New or promoted constraint; need to replan so we can enforce
            PropertyModel.prototype.recordConstraintChange = function (ccid) {
                this.needEnforcing[ccid] = true;
                this.recordChange();
            };
            //--------------------------------------------
            // Variable value has changed; need to evaluate downstream
            PropertyModel.prototype.recordVariableChange = function (stayid) {
                this.needEvaluating[stayid] = true;
                this.recordChange();
            };
            //--------------------------------------------
            // Record that update needed; schedule if appropriate
            PropertyModel.prototype.recordChange = function () {
                this.isUpdateNeeded = true;
                this.solved.set(false);
                if (this.scheduleUpdateOnChange) {
                    this.scheduleUpdate();
                }
            };
            //--------------------------------------------
            // Remove any record of constraint
            //
            PropertyModel.prototype.removeConstraintRecords = function (ccid) {
                delete this.needEnforcing[ccid];
                delete this.needEvaluating[ccid];
            };
            //--------------------------------------------
            // Remove any record of component
            PropertyModel.prototype.removeComponentRecords = function (cmp) {
                u.arraySet.remove(this.needUpdating, cmp);
            };
            /*----------------------------------------------------------------
             */
            PropertyModel.prototype.scheduleUpdate = function () {
                if (!this.isUpdateScheduled) {
                    this.isUpdateScheduled = true;
                    u.schedule(system.SystemUpdatePriority, this.performScheduledUpdate, this);
                }
            };
            PropertyModel.prototype.performScheduledUpdate = function () {
                this.isUpdateScheduled = false;
                this.update();
            };
            /*----------------------------------------------------------------
             */
            PropertyModel.prototype.update = function () {
                if (this.isUpdateNeeded) {
                    this.isUpdateNeeded = false;
                    this.commitModifications();
                    this.plan();
                    this.evaluate();
                    if (this.pendingCount == 0) {
                        this.solved.set(true);
                    }
                }
                else {
                    console.log("Update was not needed");
                }
            };
            /*----------------------------------------------------------------
             */
            PropertyModel.prototype.commitModifications = function () {
                while (this.needUpdating.length > 0) {
                    var cmp = this.needUpdating.shift();
                    var changes = m.Component.update(cmp);
                    for (var i = 0, l = changes.removes.length; i < l; ++i) {
                        this.remove(changes.removes[i]);
                    }
                    for (var i = 0, l = changes.adds.length; i < l; ++i) {
                        this.add(changes.adds[i]);
                    }
                }
            };
            /*----------------------------------------------------------------
             * Update solution graph & dependent info.
             */
            PropertyModel.prototype.plan = function () {
                var cids = u.stringSet.members(this.needEnforcing);
                if (cids.length > 0) {
                    // Run planner
                    console.log("selectedForConstraint: Running planner");
                    if (this.planner.plan(this.sgraph, cids)) {
                        console.log("selectedForConstraint: Succeeded, setting sgraph");
                        this.sgraph = this.planner.getSGraph();
                        // Topological sort of all mids and vids
                        var tgraph = system.topograph(this.cgraph, this.sgraph);
                        this.topomids = system.toposort(tgraph, this.planner);
                        var priorities = [];
                        for (var i = this.topomids.length - 1; i >= 0; --i) {
                            var cid = tgraph.constraintForMethod(this.topomids[i]);
                            if (g.isStayConstraint(cid) ||
                                this.constraints[cid].optional) {
                                priorities.push(cid);
                            }
                        }
                        // Update stay strengths
                        this.planner.setOptionals(priorities);
                        // New constraints need to be evaluated
                        cids.forEach(u.stringSet.add.bind(null, this.needEvaluating));
                        // Reevaluate any emerging source variables
                        if (hd.config.forwardEmergingSources) {
                            this.sgraph.variables().forEach(this.reevaluateIfEmergingSource, this);
                        }
                        // Update source statuses
                        this.sgraph.variables().forEach(this.updateSourceStatus, this);
                    }
                    else {
                        console.log("selectedForConstraint: Failed, did not set sgraph");
                    }
                    this.needEnforcing = {};
                }
            };
            // Helper - check for source variables that
            PropertyModel.prototype.reevaluateIfEmergingSource = function (vid) {
                var vv = this.variables[vid];
                var stayConstraintId = g.stayConstraint(vid);
                // Evaluate if it's selected AND not previously a source
                //   AND not currently scheduled for evaluation
                if (this.sgraph.selectedForConstraint(stayConstraintId) &&
                    !vv.source.get() && !this.needEvaluating[stayConstraintId]) {
                    vv.makePromise(vv.getForwardedPromise());
                    this.needEvaluating[stayConstraintId] = true;
                }
            };
            // Helper - set source property based on current solution graph
            PropertyModel.prototype.updateSourceStatus = function (vid) {
                if (this.sgraph.selectedForConstraint(g.stayConstraint(vid))) {
                    this.variables[vid].source.set(true);
                }
                else {
                    this.variables[vid].source.set(false);
                }
            };
            /*----------------------------------------------------------------
             * Run any methods which need updating.
             */
            PropertyModel.prototype.evaluate = function () {
                var cids = u.stringSet.members(this.needEvaluating);
                if (cids.length > 0) {
                    console.log("Getting mids");
                    var mids = cids
                        .map(function (cid) {
                        return this.sgraph.selectedForConstraint(cid);
                    }, this)
                        .filter(u.isNotNull);
                    var downstreamVids = new g.DigraphWalker(this.sgraph.graph)
                        .nodesDownstreamOtherType(mids);
                    // Commit initial edits
                    for (var i = 0, l = downstreamVids.length; i < l; ++i) {
                        var vid = downstreamVids[i];
                        this.variables[vid].commitPromise();
                    }
                    if (mids.length > 0) {
                        // Collect methods to be run
                        var downstreamMids = new g.DigraphWalker(this.sgraph.graph)
                            .nodesDownstreamSameType(mids)
                            .filter(g.isNotStayMethod)
                            .reduce(u.stringSet.build, {});
                        var scheduledMids = this.topomids
                            .filter(function (mid) { return downstreamMids[mid]; });
                        // Evaluate methods
                        for (var i = 0, l = scheduledMids.length; i < l; ++i) {
                            var mid = scheduledMids[i];
                            var ar = system.activate(this.methods[mid]);
                            this.enable.methodScheduled(this.cgraph.constraintForMethod(mid), mid, ar.inputs, ar.outputs);
                        }
                        // Commit all output promises
                        for (var i = 0, l = downstreamVids.length; i < l; ++i) {
                            var vid = downstreamVids[i];
                            this.variables[vid].commitPromise();
                        }
                        this.needEvaluating = {};
                    }
                }
            };
            /*----------------------------------------------------------------
             */
            PropertyModel.prototype.onNextEgraph = function (egraph) {
                var outputVids = u.stringSet.members(this.outputVids);
                if (outputVids.length) {
                    var labels = e.globalContributingCheck(this.sgraph, egraph, outputVids);
                    for (var vid in this.variables) {
                        var vv = this.variables[vid];
                        if (labels[vid] === e.Label.Relevant) {
                            vv.contributing.set(u.Fuzzy.Yes);
                            vv.relevant.set(u.Fuzzy.Yes);
                        }
                        else if (labels[vid] === e.Label.AssumedRelevant) {
                            vv.contributing.set(u.Fuzzy.Maybe);
                            vv.relevant.set(u.Fuzzy.Maybe);
                        }
                        else {
                            vv.contributing.set(u.Fuzzy.No);
                            vv.relevant.set(e.relevancyCheck(this.cgraph, this.enable.egraph, this.outputVids, vid));
                        }
                    }
                }
            };
            /*----------------------------------------------------------------
             */
            PropertyModel.prototype.scheduleCommand = function (cmd) {
                this.commandQueue.push(cmd);
                if (!this.isCommandScheduled) {
                    this.isCommandScheduled = true;
                    u.schedule(system.SystemCommandPriority, this.performCommands, this);
                }
            };
            PropertyModel.prototype.performCommands = function () {
                this.isCommandScheduled = false;
                while (this.commandQueue.length > 0) {
                    var cmd = this.commandQueue.shift();
                    var ar = system.activate(cmd);
                    for (var id in ar.outputs) {
                        if (ar.outputs[id] instanceof r.AccumulatingPromise) {
                            console.log('Found one');
                        }
                    }
                    if (cmd.external) {
                        this.update();
                    }
                }
            };
            PropertyModel.prototype.performCommand = function (cmd) {
                for (var i = 0, l = this.accumPromises.length; i < l; ++i) {
                    this.accumPromises[i].settle();
                }
                var ar = system.activate(cmd);
                this.accumPromises = [];
                for (var id in ar.outputs) {
                    var p = ar.outputs[id];
                    if (p instanceof r.AccumulatingPromise) {
                        this.accumPromises.push(p);
                    }
                }
                this.update();
            };
            return PropertyModel;
        }());
        system.PropertyModel = PropertyModel;
    })(system = hd.system || (hd.system = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var system;
    (function (system) {
        var r = hd.reactive;
        var m = hd.model;
        /*==================================================================
         * Serves as a constant function for operations that only give a
         * result.
         */
        function constantFunction(result, numOutputs) {
            if (numOutputs === void 0) { numOutputs = 1; }
            var outs = [];
            for (var i = 0; i < numOutputs; ++i) {
                outs.push(new r.Promise());
            }
            if (numOutputs == 1) {
                result = [result];
            }
            else if (!Array.isArray(result)) {
                console.error('Multi-output constant did not supply array');
                for (var i = 0; i < numOutputs; ++i) {
                    outs[i].reject(null);
                }
                return;
            }
            for (var i = 0; i < numOutputs; ++i) {
                outs[i].resolve(result[i]);
            }
            return numOutputs == 1 ? outs[0] : outs;
        }
        /*==================================================================
         */
        function activate(op) {
            var inputLookup = {};
            var priorLookup = {};
            var outputLookup = {};
            /*----------------------------------------------------------------
             * Recursive function for building parameter multi-array while
             * recording the promise retrieved from each variable.
             */
            function collectParams(inputs, priors) {
                var params = [];
                // Collect parameter promises
                for (var i = 0, l = inputs.length; i < l; ++i) {
                    // An input is either a variable, an array, or a constant
                    var input = inputs[i];
                    var prior = priors ? priors[i] : undefined;
                    if (input instanceof m.Variable) {
                        var param;
                        // If it's a variable, determine what promise to use
                        if (prior) {
                            if (!(param = priorLookup[input.id])) {
                                if (hd.config.forwardPriorGens) {
                                    param = priorLookup[input.id] = input.getForwardedPromise();
                                }
                                else {
                                    param = priorLookup[input.id] = input.getCurrentPromise();
                                }
                            }
                        }
                        else {
                            if (!(param = inputLookup[input.id])) {
                                // Make a new, dependent promise for evaluation graph
                                param = inputLookup[input.id] = new r.Promise();
                                var oldp = input.getStagedPromise();
                                if (r.plogger) {
                                    r.plogger.register(param, input.name, 'input parameter');
                                }
                                param.resolve(oldp);
                                param.ondropped.addObserver(oldp);
                            }
                        }
                        params.push(param);
                    }
                    else if (Array.isArray(input)) {
                        // If its an array, we recurse
                        params.push(collectParams(input, prior));
                    }
                    else {
                        // If it's a constant, we create a satisfied promise
                        var param = new r.Promise();
                        if (r.plogger) {
                            r.plogger.register(param, op.inputs[i], 'constant parameter');
                        }
                        param.resolve(op.inputs[i]);
                        params.push(param);
                    }
                }
                return params;
            }
            /*----------------------------------------------------------------
             * Recursive function which assigns method results to
             * corresponding variables of the output signature, while
             * recording the promise for each output variable.
             *
             * If the method returned a single promise in place of a
             * multi-array of promises, then this function (1) generates the
             * multi-array of promises required, and (2) subscribes to the
             * single promise so that when it is resolved, its value will be
             * used to resolve the generated multi-array.
             */
            function assignResults(output, result) {
                if (output instanceof m.Variable) {
                    // If we were expecting a single value...
                    if (outputLookup[output.id]) {
                        // This would only happen if an operation has the same output twice
                        //   -- i.e., an invalid method
                        console.error('Operation attempting to output twice to same variable');
                    }
                    else {
                        // Get promise
                        var p;
                        if (result instanceof r.Promise) {
                            p = result;
                        }
                        else {
                            p = new r.Promise(result);
                        }
                        if (r.plogger) {
                            r.plogger.register(p, output.name, 'output');
                        }
                        // Record promise
                        outputLookup[output.id] = p;
                        // Assign result
                        if (op.external) {
                            output.set(p);
                        }
                        else {
                            output.makePromise(p);
                        }
                    }
                }
                else if (Array.isArray(output)) {
                    // If we were expecting a multi-array...
                    var outputs = output; // just to avoid type-casts
                    // If we got an array, then recursively match the promises
                    if (Array.isArray(result)) {
                        var results = result; // just to avoid type-casts
                        for (var i = 0, l = outputs.length; i < l; ++i) {
                            assignResults(outputs[i], results[i]);
                        }
                    }
                    else {
                        // Then we resolve the multi-array using the value of the single promise
                        var p;
                        if (result instanceof r.Promise) {
                            p = result;
                        }
                        else {
                            p = new r.Promise(result);
                            if (r.plogger) {
                                r.plogger.register(p, '?', 'output placeholder');
                            }
                        }
                        var promises = generatePromises(outputs);
                        p.addDependency(null, resolveMany(promises), rejectMany(promises), null);
                    }
                }
            }
            /*----------------------------------------------------------------
             * Generates a multi-array of promises which is parallel to the
             * provided multi-array of variables: one promise for each
             * variable
             */
            function generatePromises(output) {
                if (output instanceof m.Variable) {
                    if (outputLookup[output.id]) {
                        console.error('Operation attempting to output twice to same variable');
                    }
                    else {
                        var p = outputLookup[output.id] = new r.Promise();
                        if (r.plogger) {
                            r.plogger.register(p, output.name, 'output');
                        }
                        // Assign result
                        if (op.external) {
                            output.set(p);
                        }
                        else {
                            output.makePromise(p);
                        }
                        return p;
                    }
                }
                else {
                    var outputs = output; // just to avoid type-casts
                    var ps = [];
                    for (var i = 0, l = outputs.length; i < l; ++i) {
                        ps[i] = generatePromises(outputs[i]);
                    }
                    return ps;
                }
            }
            /*----------------------------------------------------------------
             * Body of activate
             */
            // Get the parameters
            var params = collectParams(op.inputs, op.priorFlags);
            // Invoke the operation
            try {
                var result;
                if (op.fn) {
                    result = op.fn.apply(null, params);
                }
                else {
                    result = op.result;
                }
            }
            catch (e) {
                console.error(e);
                // Create failed promises for outputs
                result = [];
                for (var i = 0, l = op.outputs.length; i < l; ++i) {
                    var p = new r.Promise();
                    p.reject(null);
                    result.push(p);
                }
            }
            // Match results to signature
            assignResults(op.outputs[0], result);
            return { inputs: inputLookup,
                outputs: outputLookup };
        }
        system.activate = activate;
        /*==================================================================
         * Resolves all promises in a promise multi-array with corresponding
         * value in value multi-array.  If we find a single value where we
         * were expecting an multi-array of values, then we assume it is a
         * promise and subscribe to it so that we can use its value to
         * fulfill the multi-array.  Curried to make it easier to subscribe
         * to promises.
         */
        function resolveMany(promises) {
            return function (results) {
                // It should be an array (we wouldn't use this for a single promise)
                if (Array.isArray(results)) {
                    for (var i = 0, l = promises.length; i < l; ++i) {
                        var promise = promises[i];
                        var result = results[i];
                        if (promise instanceof r.Promise) {
                            // Single promise
                            promise.resolve(result);
                        }
                        else if (Array.isArray(promise)) {
                            var subpromises = promise; // just to avoid type-casts
                            // Array of promises
                            if (Array.isArray(result)) {
                                // Array of values
                                resolveMany(subpromises)(result);
                            }
                            else {
                                // Single value
                                var p;
                                if (result instanceof r.Promise) {
                                    p = result;
                                }
                                else {
                                    p = new r.Promise(result);
                                }
                                p.addDependency(null, resolveMany(subpromises), rejectMany(subpromises), null);
                            }
                        }
                    }
                }
                else {
                    console.error("Expected array for return value of method");
                    rejectMany(promises)("Internal error");
                }
            };
        }
        function rejectMany(promises) {
            return function (reason) {
                for (var i = 0, l = promises.length; i < l; ++i) {
                    var promise = promises[i];
                    if (promise instanceof r.Promise) {
                        promise.reject(reason);
                    }
                    else {
                        rejectMany(promise)(reason);
                    }
                }
            };
        }
    })(system = hd.system || (hd.system = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var system;
    (function (system) {
        var u = hd.utility;
        var g = hd.graph;
        var m = hd.model;
        function voidMethod() {
            return m.makeId('void');
        }
        system.voidMethod = voidMethod;
        function isVoidMethod(mid) {
            return mid.substring(0, 5) === 'void#';
        }
        system.isVoidMethod = isVoidMethod;
        function topograph(cgraph, sgraph) {
            var tgraph = new g.CachingSolutionGraph;
            cgraph.variables().forEach(function (vid) {
                tgraph.addVariable(vid);
            });
            var cids = cgraph.constraints();
            for (var i = 0, l = cids.length; i < l; ++i) {
                var cid = cids[i];
                var mid = sgraph.selectedForConstraint(cid);
                if (mid) {
                    tgraph.addMethod(mid, cid, cgraph.inputsForMethod(mid), cgraph.outputsForMethod(mid));
                }
                else {
                    mid = m.makeId("void");
                    tgraph.addMethod(mid, cid, cgraph.outputsForConstraint(cid), []);
                }
            }
            return tgraph;
        }
        system.topograph = topograph;
        /*==================================================================
         * Perform topological sort of solution graph.
         * Sorts methods and variables separately.
         * Variables are subsorted on strength.
         */
        function toposort(tgraph, planner) {
            // Data structures to store the order calculated
            var mids = [];
            // Data structures to keep track of free nodes
            var freeVids = [];
            var freeMids = new u.Heap(function (a, b) {
                return planner.compare(tgraph.constraintForMethod(a), tgraph.constraintForMethod(b)) > 0;
            });
            // In-degree of all nodes
            var counts = {};
            // Get initial in-degree for methods
            tgraph.methods().forEach(function (mid) {
                var count = tgraph.inputsForMethod(mid).length;
                counts[mid] = count;
                if (count == 0) {
                    freeMids.push(mid);
                }
            });
            // Get initial in-degree for variables
            tgraph.variables().forEach(function (vid) {
                var count = tgraph.methodsWhichOutput(vid).length;
                counts[vid] = count;
                if (count == 0) {
                    freeVids.push(vid);
                }
            });
            // Reduce in-degree for method
            var reduceMid = function reduceMid(mid) {
                if (--counts[mid] == 0) {
                    freeMids.push(mid);
                }
            };
            // Reduce in-degree for variable
            var reduceVid = function reduceVid(vid) {
                if (--counts[vid] == 0) {
                    freeVids.push(vid);
                }
            };
            // Variables that are topologically unrelated should be sorted by strength
            // Repeat until graph is empty
            while (freeVids.length > 0 || freeMids.length > 0) {
                // Pick off any free variables
                while (freeVids.length > 0) {
                    var vid = freeVids.pop();
                    tgraph.methodsWhichInput(vid).forEach(reduceMid);
                }
                // Pick off just one free method
                if (freeMids.length > 0) {
                    var mid = freeMids.pop();
                    mids.push(mid);
                    tgraph.outputsForMethod(mid).forEach(reduceVid);
                }
            }
            return mids;
        }
        system.toposort = toposort;
    })(system = hd.system || (hd.system = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var s = hd.system;
    // RunTime
    hd.PropertyModel = s.PropertyModel;
})(hd || (hd = {}));
//
var __extends = (this && this.__extends) || (function () {
    var extendStatics = function (d, b) {
        extendStatics = Object.setPrototypeOf ||
            ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
            function (d, b) { for (var p in b) if (b.hasOwnProperty(p)) d[p] = b[p]; };
        return extendStatics(d, b);
    };
    return function (d, b) {
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
var hd;
(function (hd) {
    var config;
    (function (config) {
        config.bindAttr = 'data-bind';
        config.bindEnv = 'bd';
    })(config = hd.config || (hd.config = {}));
})(hd || (hd = {}));
(function (hd) {
    var bind;
    (function (bind) {
        var u = hd.utility;
        var r = hd.reactive;
        function localScope(scope) {
            var local = Object.create(scope);
            return local;
        }
        bind.localScope = localScope;
        /*==================================================================
         */
        var Direction;
        (function (Direction) {
            Direction[Direction["bi"] = 0] = "bi";
            Direction[Direction["v2m"] = 1] = "v2m";
            Direction[Direction["m2v"] = 2] = "m2v";
        })(Direction = bind.Direction || (bind.Direction = {}));
        /*==================================================================
         * Safety functions -- these are basically run-time type checks:
         * checking that objects support the expected interfaces so that
         * we can give more helpful error messages.  (Otherwise the error
         * reported will be that we tried to call an undefined funciton in
         * our code.)
         */
        // Make sure element is required HTML
        function checkHtml(el, type) {
            if (typeof el !== 'object' || !(el instanceof type)) {
                throw type.name + " required";
            }
            return el;
        }
        bind.checkHtml = checkHtml;
        // Make sure object is observer
        function isObserver(t) {
            return (typeof t === 'object' &&
                typeof t.onNext === 'function');
        }
        bind.isObserver = isObserver;
        // Make sure object is observable
        function isObservable(t) {
            return (typeof t === 'object' &&
                typeof t.addObserver === 'function' &&
                typeof t.removeObserver === 'function');
        }
        bind.isObservable = isObservable;
        // Make sure object is both observable and observer
        function isExtension(t) {
            return (typeof t === 'object' &&
                typeof t.onNext === 'function' &&
                typeof t.addObserver === 'function' &&
                typeof t.removeObserver === 'function');
        }
        bind.isExtension = isExtension;
        // Check each element of binding
        function verifyBinding(b) {
            if (!b.model) {
                throw "no model specified";
            }
            if (!b.view) {
                throw "no view specified";
            }
            if (b.dir === undefined) {
                if (isObservable(b.model) && isObserver(b.view)) {
                    if (isObservable(b.view) && isObserver(b.model)) {
                        b.dir = Direction.bi;
                    }
                    else {
                        b.dir = Direction.m2v;
                    }
                }
                else {
                    if (isObservable(b.view) && isObserver(b.model)) {
                        b.dir = Direction.v2m;
                    }
                    else {
                        throw "unable to deduce binding direction";
                    }
                }
            }
            else {
                if (b.dir != Direction.v2m) {
                    if (!isObservable(b.model)) {
                        throw "model not observable";
                    }
                    if (!isObserver(b.view)) {
                        throw "view not observer";
                    }
                }
                if (b.dir != Direction.m2v) {
                    if (!isObservable(b.view)) {
                        throw "view not observable";
                    }
                    if (!isObserver(b.model)) {
                        throw "model not observer";
                    }
                }
            }
            if (b.dir != Direction.v2m && b.toView) {
                if (Array.isArray(b.toView)) {
                    var exts = b.toView;
                    if (!exts.every(isExtension)) {
                        throw "toView contains invalid extension";
                    }
                    b.toView = new r.Chain(exts);
                }
                else {
                    if (!isExtension(b.toView)) {
                        throw "toView is not extension";
                    }
                }
            }
            if (b.dir != Direction.m2v && b.toModel) {
                if (Array.isArray(b.toModel)) {
                    var exts = b.toModel;
                    if (!exts.every(isExtension)) {
                        throw "toModel contains invalid extension";
                    }
                    b.toModel = new r.Chain(exts);
                }
                else {
                    if (!isExtension(b.toModel)) {
                        throw "toModel is not extension";
                    }
                }
            }
            return b;
        }
        /*==================================================================
         * Bind and unbind
         */
        function createBindings(b) {
            if (Array.isArray(b)) {
                b.forEach(createBindings);
            }
            else {
                bindSingle(b);
            }
        }
        bind.createBindings = createBindings;
        function bindSingle(b) {
            var vb = verifyBinding(b);
            if (vb.dir != Direction.v2m) {
                if (vb.toView) {
                    vb.toView.addObserver(vb.view);
                    vb.model.addObserver(vb.toView);
                }
                else {
                    vb.model.addObserver(vb.view);
                }
            }
            if (vb.dir != Direction.m2v) {
                if (vb.toModel) {
                    vb.view.addObserver(vb.toModel);
                    vb.toModel.addObserver(vb.model);
                }
                else {
                    vb.view.addObserver(vb.model);
                }
            }
        }
        /*----------------------------------------------------------------*/
        function destroyBindings(b) {
            if (Array.isArray(b)) {
                b.forEach(destroyBindings);
            }
            else {
                unbindSingle(b);
            }
        }
        bind.destroyBindings = destroyBindings;
        function unbindSingle(b) {
            var vb = verifyBinding(b);
            if (vb.dir != Direction.v2m) {
                if (vb.toView) {
                    vb.model.removeObserver(vb.toView);
                    vb.toView.removeObserver(vb.view);
                }
                else {
                    vb.model.removeObserver(vb.view);
                }
            }
            if (vb.dir != Direction.m2v) {
                if (vb.toModel) {
                    vb.view.removeObserver(vb.toModel);
                    vb.toModel.removeObserver(vb.model);
                }
                else {
                    vb.view.removeObserver(vb.model);
                }
            }
        }
        /*==================================================================
         * Search DOM tree for binding specifications and perform them.
         */
        /*------------------------------------------------------------------
         * Entry point
         */
        function createDeclaredBindings(scope, el) {
            if (el === void 0) { el = document.body; }
            var bindings = [];
            if (el.nodeType === Node.ELEMENT_NODE) {
                searchForBindings(el, scope, bindings);
            }
            return bindings;
        }
        bind.createDeclaredBindings = createDeclaredBindings;
        /*------------------------------------------------------------------
         * Recursive search function.
         */
        function searchForBindings(el, scope, bindings) {
            // Look for declarative binding specification
            var spec = el.getAttribute(hd.config.bindAttr);
            var halt = false;
            if (spec) {
                halt = bindElement(spec, el, scope, bindings);
            }
            if (!halt) {
                for (var i = 0, l = el.childNodes.length; i < l; ++i) {
                    if (el.childNodes[i].nodeType === Node.ELEMENT_NODE) {
                        searchForBindings(el.childNodes[i], scope, bindings);
                    }
                }
            }
        }
        /*------------------------------------------------------------------
         * Attempt to bind one single element from specification.
         */
        function bindElement(spec, el, scope, bindings) {
            // Eval binding string as JS
            try {
                var functionBody = compile(spec);
                var elBindingsFn = new Function(hd.config.bindEnv, functionBody);
                var env = new bind.BindEnvironment(el, scope);
                var elNestedBindings = elBindingsFn.call(null, env);
            }
            catch (e) {
                console.error("Invalid binding declaration: " + spec, e);
                return false;
            }
            var halt = false;
            var i = 0;
            u.multiArray.forEach(elNestedBindings, function (b) {
                halt = halt || b.halt;
                try {
                    bindSingle(b);
                    bindings.push(b);
                }
                catch (e) {
                    console.error('Invalid binding ' + i + ': ' + spec, e);
                }
                ++i;
            });
            return halt;
        }
        /*------------------------------------------------------------------
         * Compile binding specification string to function which produces
         * the specification object.
         *
         * Very straightforward for now.  Later we might try to support some
         * of the constructs John implemented.
         */
        function compile(spec) {
            return "with (" + hd.config.bindEnv + ".scope) {" +
                "  return [" + spec + "]" +
                "}";
        }
    })(bind = hd.bind || (hd.bind = {}));
})(hd || (hd = {}));
/*####################################################################
 * Binding which simply inserts value as text in an element.
 */
var hd;
(function (hd) {
    var bind;
    (function (bind) {
        /*==================================================================
         * Observer for binding
         */
        var Text = /** @class */ (function () {
            // initialize
            function Text(el) {
                this.el = bind.checkHtml(el, HTMLElement);
            }
            /*----------------------------------------------------------------
             * Observe variable
             */
            Text.prototype.onNext = function (value) {
                if (value === undefined || value === null) {
                    value = '';
                }
                var el = this.el;
                while (el.lastChild) {
                    el.removeChild(el.lastChild);
                }
                el.appendChild(document.createTextNode(value));
            };
            Text.prototype.onError = function () { };
            Text.prototype.onCompleted = function () { };
            return Text;
        }());
        bind.Text = Text;
    })(bind = hd.bind || (hd.bind = {}));
})(hd || (hd = {}));
/*####################################################################
 * Binding for a text input box.
 */
var hd;
(function (hd) {
    var bind;
    (function (bind) {
        var r = hd.reactive;
        var i = 0;
        var flush = new Object();
        /*================================================================
         * Observer/Observable for binding
         */
        var Edit = /** @class */ (function (_super) {
            __extends(Edit, _super);
            /*----------------------------------------------------------------
             * Initialize and subscribe to HTML editing events.
             */
            function Edit(el) {
                var _this = _super.call(this) || this;
                // value to change from on blur
                _this.blurFrom = null;
                // value to change to on blur
                _this.blurTo = null;
                // whether we've set at least one value
                _this.initialized = false;
                _this.el = bind.checkHtml(el, HTMLInputElement);
                el.addEventListener('input', _this.update.bind(_this));
                el.addEventListener('change', _this.onBlur.bind(_this));
                return _this;
            }
            /*----------------------------------------------------------------
             * When widget is modified.
             */
            Edit.prototype.update = function () {
                this.initialized = true;
                this.sendNext(this.el.value);
            };
            /*----------------------------------------------------------------
             * When variable is modified.
             */
            Edit.prototype.onNext = function (value) {
                if (value === undefined || value === null) {
                    value = '';
                }
                else if (typeof value !== 'string') {
                    // value = value.toString();
                }
                // the basic idea here is this:  the value should not change as you
                // are editing; it should wait and change when you are through editing
                if (this.initialized && this.el === document.activeElement) {
                    this.blurTo = value;
                }
                else {
                    if (this.el.value != value) {
                        this.el.value = value;
                        if (this.el === document.activeElement) {
                            this.el.select();
                        }
                    }
                    this.blurTo = null;
                }
            };
            /*----------------------------------------------------------------
             * Perform any changes which were quashed during editing
             */
            Edit.prototype.onBlur = function () {
                if (this.blurTo !== null && this.el.value != this.blurTo) {
                    this.el.value = this.blurTo;
                }
                this.blurTo = null;
                this.initialized = false;
                this.sendCompleted();
            };
            /*----------------------------------------------------------------
             */
            Edit.prototype.onError = function (value) { };
            /*----------------------------------------------------------------
             */
            Edit.prototype.onCompleted = function () { };
            return Edit;
        }(r.BasicObservable));
        bind.Edit = Edit;
    })(bind = hd.bind || (hd.bind = {}));
})(hd || (hd = {}));
/*####################################################################
 * Binding for CSS class name.  Takes boolean observable and two
 * css class names: one for when the observable is true, and one for
 * when it's false.  If a class name evaluates to false, then no
 * class is used for that value.
 */
var hd;
(function (hd) {
    var bind;
    (function (bind) {
        var CssClass = /** @class */ (function () {
            /*----------------------------------------------------------------
             * Initialize
             */
            function CssClass(el, trueClass, falseClass) {
                this.el = el;
                this.trueClass = trueClass;
                this.falseClass = falseClass;
            }
            /*----------------------------------------------------------------
             * Observe variable
             */
            CssClass.prototype.onNext = function (value) {
                if (value) {
                    if (this.falseClass) {
                        this.el.classList.remove(this.falseClass);
                    }
                    if (this.trueClass) {
                        this.el.classList.add(this.trueClass);
                    }
                }
                else {
                    if (this.trueClass) {
                        this.el.classList.remove(this.trueClass);
                    }
                    if (this.falseClass) {
                        this.el.classList.add(this.falseClass);
                    }
                }
            };
            CssClass.prototype.onError = function () { };
            CssClass.prototype.onCompleted = function () { };
            return CssClass;
        }());
        bind.CssClass = CssClass;
    })(bind = hd.bind || (hd.bind = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var bind;
    (function (bind) {
        var BasicObservable = hd.reactive.BasicObservable;
        var Options = /** @class */ (function () {
            function Options(el) {
                this.el = el;
            }
            Options.prototype.onNext = function (value) {
                var el = this.el;
                while (el.lastChild) {
                    el.removeChild(el.lastChild);
                }
                value.forEach(function (entry) {
                    var option = document.createElement('option');
                    option.value = entry.id;
                    option.text = entry.label ? entry.label : entry.id;
                    el.appendChild(option);
                });
                var evt = document.createEvent('HTMLEvents');
                evt.initEvent("change", false, true);
                this.el.dispatchEvent(evt);
            };
            Options.prototype.onError = function () { };
            Options.prototype.onCompleted = function () { };
            return Options;
        }());
        /*******************************************************************
         */
        var Value = /** @class */ (function (_super) {
            __extends(Value, _super);
            function Value(el) {
                var _this = _super.call(this) || this;
                _this.el = bind.checkHtml(el, HTMLElement);
                if (el) {
                    var boundUpdate = _this.update.bind(_this);
                    el.addEventListener('input', boundUpdate);
                    // el.addEventListener( 'change', boundUpdate );
                }
                return _this;
            }
            Value.prototype.update = function () {
                this.sendNext(this.el.value);
            };
            Value.prototype.onNext = function (value) {
                if (value) {
                    this.el.value = value;
                }
            };
            Value.prototype.onError = function () { };
            Value.prototype.onCompleted = function () { };
            return Value;
        }(BasicObservable));
        bind.Value = Value;
    })(bind = hd.bind || (hd.bind = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var bind;
    (function (bind) {
        var r = hd.reactive;
        var Checked = /** @class */ (function (_super) {
            __extends(Checked, _super);
            function Checked(el) {
                var _this = _super.call(this) || this;
                _this.el = bind.checkHtml(el, HTMLInputElement);
                var boundUpdate = _this.update.bind(_this);
                el.addEventListener('input', boundUpdate);
                el.addEventListener('change', boundUpdate);
                return _this;
            }
            Checked.prototype.update = function () {
                this.sendNext(this.el.checked);
            };
            Checked.prototype.onNext = function (value) {
                this.el.checked = !!value;
            };
            Checked.prototype.onError = function () { };
            Checked.prototype.onCompleted = function () { };
            return Checked;
        }(r.BasicObservable));
        bind.Checked = Checked;
    })(bind = hd.bind || (hd.bind = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var bind;
    (function (bind) {
        var Enabled = /** @class */ (function () {
            function Enabled(el) {
                this.el = bind.checkHtml(el, HTMLElement);
            }
            Enabled.prototype.onNext = function (value) {
                if (value) {
                    this.el.disabled = false;
                }
                else {
                    this.el.disabled = true;
                }
            };
            Enabled.prototype.onError = function () { };
            Enabled.prototype.onCompleted = function () { };
            return Enabled;
        }());
        bind.Enabled = Enabled;
    })(bind = hd.bind || (hd.bind = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var bind;
    (function (bind) {
        var r = hd.reactive;
        var MousePosition = /** @class */ (function (_super) {
            __extends(MousePosition, _super);
            function MousePosition() {
                var _this = _super.call(this) || this;
                var that = _this;
                document.addEventListener('mousemove', function (event) {
                    that.pos = { x: event.clientX, y: event.clientY };
                    that.sendNext(that.pos);
                });
                return _this;
            }
            MousePosition.prototype.get = function () {
                return this.pos;
            };
            return MousePosition;
        }(r.BasicObservable));
        bind.MousePosition = MousePosition;
        var theMousePosition;
        function getMousePosition() {
            if (!theMousePosition) {
                theMousePosition = new MousePosition();
            }
            return theMousePosition;
        }
        bind.getMousePosition = getMousePosition;
    })(bind = hd.bind || (hd.bind = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var bind;
    (function (bind) {
        var Position = /** @class */ (function () {
            function Position(el) {
                this.el = bind.checkHtml(el, HTMLElement);
            }
            Position.prototype.onNext = function (p) {
                this.el.style.left = p.x + 'px';
                this.el.style.top = p.y + 'px';
            };
            Position.prototype.onError = function () { };
            Position.prototype.onCompleted = function () { };
            return Position;
        }());
        bind.Position = Position;
    })(bind = hd.bind || (hd.bind = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var bind;
    (function (bind) {
        var r = hd.reactive;
        var MouseDown = /** @class */ (function (_super) {
            __extends(MouseDown, _super);
            function MouseDown(el) {
                var _this = _super.call(this) || this;
                _this.el = bind.checkHtml(el, HTMLElement);
                _this.el.addEventListener('mousedown', _this.onmousedown.bind(_this));
                return _this;
            }
            MouseDown.prototype.onmousedown = function (e) {
                this.sendNext(e);
            };
            return MouseDown;
        }(r.BasicObservable));
        bind.MouseDown = MouseDown;
        var MouseUp = /** @class */ (function (_super) {
            __extends(MouseUp, _super);
            function MouseUp(el) {
                var _this = _super.call(this) || this;
                _this.el = bind.checkHtml(el, HTMLElement);
                _this.el.addEventListener('mouseup', _this.onmouseup.bind(_this));
                return _this;
            }
            MouseUp.prototype.onmouseup = function (e) {
                this.sendNext(e);
            };
            return MouseUp;
        }(r.BasicObservable));
        bind.MouseUp = MouseUp;
        var Click = /** @class */ (function (_super) {
            __extends(Click, _super);
            function Click(el) {
                var _this = _super.call(this) || this;
                _this.el = bind.checkHtml(el, HTMLElement);
                _this.el.addEventListener('click', _this.onclick.bind(_this));
                return _this;
            }
            Click.prototype.onclick = function (e) {
                this.sendNext(e);
            };
            return Click;
        }(r.BasicObservable));
        bind.Click = Click;
        var DblClick = /** @class */ (function (_super) {
            __extends(DblClick, _super);
            function DblClick(el) {
                var _this = _super.call(this) || this;
                _this.el = bind.checkHtml(el, HTMLElement);
                _this.el.addEventListener('dblclick', _this.ondblclick.bind(_this));
                return _this;
            }
            DblClick.prototype.ondblclick = function (e) {
                this.sendNext(e);
            };
            return DblClick;
        }(r.BasicObservable));
        bind.DblClick = DblClick;
    })(bind = hd.bind || (hd.bind = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var bind;
    (function (bind) {
        var r = hd.reactive;
        var KeyDown = /** @class */ (function (_super) {
            __extends(KeyDown, _super);
            function KeyDown(el) {
                var _this = _super.call(this) || this;
                _this.el = bind.checkHtml(el, HTMLElement);
                _this.el.addEventListener('keydown', _this.onkeydown.bind(_this));
                return _this;
            }
            KeyDown.prototype.onkeydown = function (e) {
                this.sendNext(e);
            };
            return KeyDown;
        }(r.BasicObservable));
        bind.KeyDown = KeyDown;
        var Change = /** @class */ (function (_super) {
            __extends(Change, _super);
            function Change(el) {
                var _this = _super.call(this) || this;
                _this.el = bind.checkHtml(el, HTMLElement);
                _this.el.addEventListener('change', _this.onchange.bind(_this));
                return _this;
            }
            Change.prototype.onchange = function (e) {
                this.sendNext(e);
            };
            return Change;
        }(r.BasicObservable));
        bind.Change = Change;
        var OnlyKey = /** @class */ (function (_super) {
            __extends(OnlyKey, _super);
            function OnlyKey(keyCode) {
                var _this = _super.call(this) || this;
                _this.keyCode = keyCode;
                return _this;
            }
            OnlyKey.prototype.onNext = function (e) {
                if (e.keyCode == this.keyCode) {
                    this.sendNext(e);
                }
            };
            return OnlyKey;
        }(r.Extension));
        bind.OnlyKey = OnlyKey;
    })(bind = hd.bind || (hd.bind = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var bind;
    (function (bind) {
        var r = hd.reactive;
        var Time = /** @class */ (function (_super) {
            __extends(Time, _super);
            function Time(time_ms) {
                var _this = _super.call(this) || this;
                window.setInterval(_this.update.bind(_this), time_ms);
                return _this;
            }
            Time.prototype.update = function () {
                this.sendNext(new Date());
            };
            return Time;
        }(r.BasicObservable));
        bind.Time = Time;
    })(bind = hd.bind || (hd.bind = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var bind;
    (function (bind) {
        var r = hd.reactive;
        var ForEach = /** @class */ (function () {
            function ForEach(el, scope, name, idxName) {
                this.body = [];
                this.name = name;
                this.idxName = idxName;
                this.root = bind.checkHtml(el, HTMLElement);
                this.scope = scope;
                for (var i = el.childNodes.length - 1; i >= 0; --i) {
                    this.body[i] = el.childNodes[i];
                    el.removeChild(el.childNodes[i]);
                }
            }
            ForEach.prototype.onNext = function (values) {
                while (this.root.lastChild) {
                    this.root.removeChild(this.root.lastChild);
                }
                for (var i = 0, l = values.length; i < l; ++i) {
                    var v = values[i];
                    var local = bind.localScope(this.scope);
                    var el = local['$' + this.name] = new r.Constant(null);
                    if (this.idxName) {
                        var idx = local['$' + this.idxName] = new r.Constant(0);
                    }
                    for (var j = 0, m = this.body.length; j < m; ++j) {
                        var n = this.body[j].cloneNode(true);
                        this.root.appendChild(n);
                        local[this.name] = v;
                        el.value = v;
                        if (this.idxName) {
                            local[this.idxName] = i;
                            idx.value = i;
                        }
                        bind.createDeclaredBindings(local, n);
                    }
                }
            };
            ForEach.prototype.onError = function () { };
            ForEach.prototype.onCompleted = function () { };
            return ForEach;
        }());
        bind.ForEach = ForEach;
    })(bind = hd.bind || (hd.bind = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var bind;
    (function (bind) {
        var When = /** @class */ (function () {
            function When(el) {
                this.el = bind.checkHtml(el, HTMLElement);
            }
            When.prototype.onNext = function (value) {
                if (value) {
                    this.el.style.removeProperty('display');
                }
                else {
                    this.el.style.display = 'none';
                }
            };
            When.prototype.onError = function () { };
            When.prototype.onCompleted = function () { };
            return When;
        }());
        bind.When = When;
    })(bind = hd.bind || (hd.bind = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var bind;
    (function (bind) {
        var r = hd.reactive;
        var m = hd.model;
        function concat(as, bs) {
            if (!as) {
                return bs;
            }
            if (!bs) {
                return as;
            }
            if (Array.isArray(as)) {
                if (Array.isArray(bs)) {
                    return as.concat(bs);
                }
                else {
                    return as.concat([bs]);
                }
            }
            else {
                if (Array.isArray(bs)) {
                    return [as].concat(bs);
                }
                else {
                    return [as, bs];
                }
            }
        }
        var BindEnvironment = /** @class */ (function () {
            function BindEnvironment(el, scope) {
                this.el = el;
                this.scope = scope;
            }
            BindEnvironment.prototype.edit = function (target, toView, toModel) {
                return {
                    view: new bind.Edit(this.el),
                    model: target,
                    dir: bind.Direction.bi,
                    toView: toView,
                    toModel: concat(toModel, new r.Stabilizer())
                };
            };
            BindEnvironment.prototype.editVar = function (vv, toView, toModel) {
                return [
                    this.edit(vv, toView, toModel),
                    this.cssClass(vv),
                    this.enabled(vv.relevant)
                ];
            };
            BindEnvironment.prototype.num = function (target, places, toView, toModel) {
                if (places === undefined || places === null) {
                    return {
                        view: new bind.Edit(this.el),
                        model: target,
                        dir: bind.Direction.bi,
                        toView: toView,
                        toModel: concat(concat(new r.ToNumber(), toModel), new r.Stabilizer())
                    };
                }
                else {
                    return {
                        view: new bind.Edit(this.el),
                        model: target,
                        dir: bind.Direction.bi,
                        toView: concat(toView, places >= 0 ? new r.NumberToFixed(places) : new r.Round(places)),
                        toModel: concat(concat([new r.ToNumber(), new r.Round(places)], toModel), new r.Stabilizer())
                    };
                }
            };
            BindEnvironment.prototype.numVar = function (vv, places, toView, toModel) {
                return [
                    this.num(vv, places, toView, toModel),
                    this.cssClass(vv),
                    this.enabled(vv.relevant)
                ];
            };
            BindEnvironment.prototype.date = function (target, toView, toModel) {
                return {
                    view: new bind.Edit(this.el),
                    model: target,
                    dir: bind.Direction.bi,
                    toView: concat(toView, new r.DateToDateString()),
                    toModel: concat(concat(new r.ToDate(), toModel), new r.Stabilizer())
                };
            };
            BindEnvironment.prototype.dateVar = function (vv, toView, toModel) {
                return [
                    this.date(vv, toView, toModel),
                    this.cssClass(vv),
                    this.enabled(vv.relevant)
                ];
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.text = function (target, toView) {
                return {
                    view: new bind.Text(this.el),
                    model: target,
                    toView: toView,
                    dir: bind.Direction.m2v
                };
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.cssClass = function (target, ontrue, onfalse, toView) {
                if (ontrue || onfalse || !(target instanceof m.Variable)) {
                    return {
                        view: new bind.CssClass(this.el, ontrue, onfalse),
                        model: target,
                        dir: bind.Direction.m2v,
                        toView: toView
                    };
                }
                else {
                    var vv = target;
                    return [
                        { view: new bind.CssClass(this.el, 'source', 'derived'),
                            model: vv.source,
                            dir: bind.Direction.m2v },
                        { view: new bind.CssClass(this.el, 'stale', 'current'),
                            model: vv.stale,
                            dir: bind.Direction.m2v },
                        { view: new bind.CssClass(this.el, 'pending', 'complete'),
                            model: vv.pending,
                            dir: bind.Direction.m2v },
                        { view: new bind.CssClass(this.el, 'contributing', 'noncontributing'),
                            model: vv.contributing,
                            dir: bind.Direction.m2v },
                        { view: new bind.CssClass(this.el, 'error', null),
                            model: vv.error,
                            dir: bind.Direction.m2v }
                    ];
                }
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.enabled = function (target, toView) {
                return {
                    view: new bind.Enabled(this.el),
                    model: target,
                    dir: bind.Direction.m2v,
                    toView: toView
                };
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.value = function (target, toView, toModel) {
                return {
                    view: new bind.Value(this.el),
                    model: target,
                    dir: bind.Direction.bi,
                    toView: toView,
                    toModel: toModel
                };
            };
            BindEnvironment.prototype.checked = function (target, toView, toModel) {
                return {
                    view: new bind.Checked(this.el),
                    model: target,
                    dir: bind.Direction.bi,
                    toView: toView,
                    toModel: toModel
                };
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.mouseposition = function (target, toModel) {
                return {
                    view: bind.getMousePosition(),
                    model: target,
                    dir: bind.Direction.v2m,
                    toModel: toModel
                };
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.mousedown = function (target, toModel) {
                return {
                    view: new bind.MouseDown(this.el),
                    model: target,
                    dir: bind.Direction.v2m,
                    toModel: toModel
                };
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.mouseup = function (target, toModel) {
                return {
                    view: new bind.MouseUp(this.el),
                    model: target,
                    dir: bind.Direction.v2m,
                    toModel: toModel
                };
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.click = function (target, toModel) {
                return {
                    view: new bind.Click(this.el),
                    model: target,
                    dir: bind.Direction.v2m,
                    toModel: toModel
                };
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.dblclick = function (target, toModel) {
                return {
                    view: new bind.DblClick(this.el),
                    model: target,
                    dir: bind.Direction.v2m,
                    toModel: toModel
                };
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.position = function (target, toView) {
                return {
                    view: new bind.Position(this.el),
                    model: target,
                    dir: bind.Direction.m2v,
                    toView: toView
                };
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.forEach = function (target, name, idx) {
                return {
                    view: new bind.ForEach(this.el, this.scope, name, idx),
                    model: target,
                    dir: bind.Direction.m2v,
                    halt: true
                };
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.when = function (target, toView) {
                return {
                    view: new bind.When(this.el),
                    model: target,
                    dir: bind.Direction.m2v,
                    toView: toView
                };
            };
            BindEnvironment.prototype.chain = function () {
                if (arguments.length == 0) {
                    return;
                }
                else if (arguments.length == 1) {
                    var e = arguments[0];
                    if (Array.isArray(e)) {
                        if (!e.every(bind.isExtension)) {
                            throw "Invalid extension passed to chain";
                        }
                        return new r.Chain(e);
                    }
                    else {
                        return e;
                    }
                }
                else {
                    var es = [];
                    for (var i = 0, l = arguments.length; i < l; ++i) {
                        var e = arguments[i];
                        if (Array.isArray(e)) {
                            if (!e.every(bind.isExtension)) {
                                throw "Invalid extension passed to chain";
                            }
                            Array.prototype.push.apply(es, e);
                        }
                        else if (e) {
                            if (!bind.isExtension(e)) {
                                throw "Invalid extension passed to chain";
                            }
                            es.push(e);
                        }
                    }
                    if (es.length > 0) {
                        return new r.Chain(es);
                    }
                    else {
                        return;
                    }
                }
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.path = function (model, name) {
                return new r.HotSwap(new m.PathValue(new m.Path(model, name)));
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.rw = function (read, write) {
                return new r.ReadWrite(read, write);
            };
            BindEnvironment.prototype.fn = function () {
                if (typeof arguments[0] === 'function') {
                    return new r.FunctionExtension(arguments[0], null, Array.prototype.slice.call(arguments, 1));
                }
                else {
                    return new r.FunctionExtension(arguments[1], arguments[0], Array.prototype.slice.call(arguments, 2));
                }
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.cn = function (value) {
                return new r.Constant(value);
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.delay = function (time_ms) {
                return new r.Delay(time_ms);
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.stabilize = function (time_ms) {
                return new r.Stabilizer(time_ms);
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.msg = function (message) {
                return new r.ReplaceError(message);
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.req = function () {
                return new r.Required();
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.def = function (value) {
                return new r.Default(value);
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.round = function (places) {
                return new r.Round(places);
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.fix = function (places) {
                return new r.NumberToFixed(places);
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.prec = function (sigfigs) {
                return new r.NumberToPrecision(sigfigs);
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.exp = function (places) {
                return new r.NumberToExponential(places);
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.scale = function (factor) {
                return new r.ScaleNumber(factor);
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.toStr = function () {
                return new r.ToString();
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.toJson = function () {
                return new r.ToJson();
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.toDate = function () {
                return new r.ToDate();
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.dateToString = function () {
                return new r.DateToString();
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.dateToDateString = function () {
                return new r.DateToDateString();
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.dateToTimeString = function () {
                return new r.DateToTimeString();
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.dateToMilliseconds = function () {
                return new r.DateToMilliseconds();
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.millisecondsToDate = function () {
                return new r.MillisecondsToDate();
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.toNum = function () {
                return new r.ToNumber();
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.offset = function (dx, dy) {
                return new r.Offset(dx, dy);
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.pointToString = function () {
                return new r.PointToString();
            };
            /*--------------------------------------------------------------*/
            BindEnvironment.prototype.or = function () {
                var observables = [];
                for (var _i = 0; _i < arguments.length; _i++) {
                    observables[_i] = arguments[_i];
                }
                return new r.Or(observables);
            };
            return BindEnvironment;
        }());
        bind.BindEnvironment = BindEnvironment;
    })(bind = hd.bind || (hd.bind = {}));
})(hd || (hd = {}));
var hd;
(function (hd) {
    var b = hd.bind;
    // Export
    hd.createBindings = b.createBindings;
    hd.destroyBindings = b.destroyBindings;
    hd.createDeclaredBindings = b.createDeclaredBindings;
    hd.Direction = b.Direction;
    hd.isObservable = b.isObservable;
    hd.isObserver = b.isObserver;
    hd.isExtension = b.isExtension;
    hd.getMousePosition = b.getMousePosition;
    // Bindings
    hd.Change = b.Change;
    hd.Checked = b.Checked;
    hd.Click = b.Click;
    hd.CssClass = b.CssClass;
    hd.DblClick = b.DblClick;
    hd.Edit = b.Edit;
    hd.Enabled = b.Enabled;
    hd.KeyDown = b.KeyDown;
    hd.MouseDown = b.MouseDown;
    hd.MouseUp = b.MouseUp;
    hd.MousePosition = b.MousePosition;
    hd.Position = b.Position;
    hd.Text = b.Text;
    hd.Time = b.Time;
    hd.Value = b.Value;
    hd.OnlyKey = b.OnlyKey;
    function bindEnv(el, scope) {
        return new b.BindEnvironment(el, scope);
    }
    hd.bindEnv = bindEnv;
})(hd || (hd = {}));
//
var hd;
(function (hd) {
    var async;
    (function (async) {
        var r = hd.reactive;
        /*==================================================================
         * Represents a pool of workers.  Tasks are assigned to a worker
         * as they become available.
         */
        var WorkerPool = /** @class */ (function () {
            /*----------------------------------------------------------------
             * Initialize.
             */
            function WorkerPool(sourceUrl, max) {
                if (max === void 0) { max = 20; }
                // List of workers not currently doing anything
                this.availableWorkers = [];
                // List of tasks currently being run by a worker
                this.runningTasks = [];
                // List of tasks waiting to be run by a worker
                this.queuedTasks = [];
                this.sourceUrl = sourceUrl;
                this.max = max;
            }
            /*----------------------------------------------------------------
             * Schedule a function to be run on the next available worker.
             */
            WorkerPool.prototype.schedule = function (fnName, inputs, numOutputs) {
                if (numOutputs === void 0) { numOutputs = 1; }
                // Create output promises
                var outputs = [];
                for (var i = 0; i < numOutputs; ++i) {
                    outputs.push(new r.Promise());
                }
                // Create task
                var task = { fnName: fnName, inputs: inputs, outputs: outputs };
                // Subscribe to dropped outputs
                outputs.forEach(function (p) {
                    p.ondropped.addObserver(this, this.onPromiseDropped, null, null, task);
                }, this);
                // Queue task, then try to run
                this.queuedTasks.push(task);
                this.checkQueue();
                return outputs;
            };
            /*----------------------------------------------------------------
             * Check to see if any queued tasks can be executed.
             */
            WorkerPool.prototype.checkQueue = function () {
                // Try using available workers
                while (this.queuedTasks.length > 0 && this.availableWorkers.length > 0) {
                    this.execute(this.queuedTasks.shift(), this.availableWorkers.shift());
                }
                // Try using new workers
                while (this.queuedTasks.length > 0 && this.runningTasks.length < this.max) {
                    var task = this.queuedTasks.shift();
                    try {
                        this.execute(task, new Worker(this.sourceUrl));
                    }
                    catch (e) {
                        console.error('Unable to create worker', e);
                        task.outputs.forEach(function (p) {
                            p.reject('Script unable to create worker');
                        });
                    }
                }
            };
            /*----------------------------------------------------------------
             * Execute a task on a worker.
             */
            WorkerPool.prototype.execute = function (task, worker) {
                task.worker = worker;
                this.runningTasks.push(task);
                worker.onmessage = this.onMessage.bind(this, task);
                worker.onerror = this.onError.bind(this, task);
                worker.postMessage({
                    fnName: task.fnName,
                    inputs: task.inputs
                });
            };
            /*----------------------------------------------------------------
             * Process normal messages from worker.
             */
            WorkerPool.prototype.onMessage = function (task, event) {
                if (event.data.error) {
                    // Failure - function failed but worker is still OK
                    console.warn('Task failed: ' + JSON.stringify(event.data.error));
                    task.outputs.forEach(function (p) {
                        p.reject(event.data.error);
                    });
                    this.returnWorker(task.worker);
                }
                else if (event.data.complete) {
                    // Completion
                    var result = event.data.result;
                    if (task.outputs.length == 1) {
                        task.outputs[0].resolve(result);
                    }
                    else {
                        for (var i = 0, l = task.outputs.length; i < l; ++i) {
                            task.outputs[i].resolve(result[i]);
                        }
                    }
                    this.returnWorker(task.worker);
                }
                else {
                    // Partial update
                    var result = event.data.result;
                    if (task.outputs.length == 1) {
                        task.outputs[0].notify(result);
                    }
                    else {
                        for (var i = 0, l = task.outputs.length; i < l; ++i) {
                            task.outputs[i].notify(result[i]);
                        }
                    }
                }
            };
            /*----------------------------------------------------------------
             * Process error from worker.  Shouldn't happen -- assume
             * something is wrong with the worker.
             */
            WorkerPool.prototype.onError = function (task, event) {
                console.warn('Worker failed: ' + JSON.stringify(event.data));
                task.outputs.forEach(function (p) {
                    p.reject(event.data);
                });
                this.killWorker(task.worker);
            };
            /*----------------------------------------------------------------
             * Return worker after task is completed.  Checks to see if there
             * are any tasks waiting on a worker; if not, it is returned to
             * the available state.
             */
            WorkerPool.prototype.returnWorker = function (worker) {
                if (this.runningTasks.length >= this.max) {
                    this.killWorker(worker);
                }
                else if (this.queuedTasks.length > 0) {
                    this.execute(this.queuedTasks.shift(), worker);
                }
                else {
                    var i = this.findTaskIndexFor(worker);
                    if (i >= 0) {
                        this.runningTasks.splice(i, 1);
                        worker.onmessage = null;
                        worker.onerror = null;
                        this.availableWorkers.push(worker);
                    }
                }
            };
            /*----------------------------------------------------------------
             * Terminates worker process; discards worker.
             */
            WorkerPool.prototype.killWorker = function (worker) {
                var i = this.findTaskIndexFor(worker);
                if (i >= 0) {
                    this.runningTasks.splice(i, 1);
                }
                worker.terminate();
                this.checkQueue();
            };
            /*----------------------------------------------------------------
             * Called when an output promise is dropped.  If all outputs are
             * dropped then it kills the worker.
             */
            WorkerPool.prototype.onPromiseDropped = function (promise, task) {
                if (task.outputs.every(isDropped)) {
                    if (task.worker) {
                        this.killWorker(task.worker);
                    }
                    else {
                        this.dequeue(task);
                    }
                }
            };
            /*----------------------------------------------------------------
             * Remove task from queue -- it's output is no longer needed.
             */
            WorkerPool.prototype.dequeue = function (task) {
                var i = this.queuedTasks.indexOf(task);
                if (i >= 0) {
                    this.queuedTasks.splice(i, 1);
                }
            };
            /*----------------------------------------------------------------
             * Find the task for given worker.
             */
            WorkerPool.prototype.findTaskIndexFor = function (worker) {
                for (var i = 0, l = this.runningTasks.length; i < l; ++i) {
                    if (this.runningTasks[i].worker === worker) {
                        return i;
                    }
                }
                return -1;
            };
            return WorkerPool;
        }());
        async.WorkerPool = WorkerPool;
        function isDropped(p) {
            return !p.hasDependencies();
        }
        /*==================================================================
         */
        async.workerPools = {};
        function setMaxWorkers(sourceUrl, max) {
            var pool = async.workerPools[sourceUrl];
            if (pool) {
                pool.max = max;
            }
            else {
                async.workerPools[sourceUrl] = new WorkerPool(sourceUrl, max);
            }
        }
        async.setMaxWorkers = setMaxWorkers;
        function worker(sourceUrl, fnName, numOutputs) {
            if (numOutputs === void 0) { numOutputs = 1; }
            if (!async.workerPools[sourceUrl]) {
                async.workerPools[sourceUrl] = new WorkerPool(sourceUrl);
            }
            return function () {
                var pool = async.workerPools[sourceUrl];
                var outputs = pool.schedule(fnName, Array.prototype.slice.call(arguments, 0, arguments.length), numOutputs);
                if (numOutputs == 1) {
                    return outputs[0];
                }
                else {
                    return outputs;
                }
            };
        }
        async.worker = worker;
    })(async = hd.async || (hd.async = {}));
})(hd || (hd = {}));
(function (hd) {
    hd.setMaxWorkers = hd.async.setMaxWorkers;
    hd.worker = hd.async.worker;
})(hd || (hd = {}));
var hd;
(function (hd) {
    var async;
    (function (async) {
        var r = hd.reactive;
        function makeQueryString(data) {
            var params = [];
            for (var key in data) {
                params.push(key + '=' + encodeURIComponent(data[key]));
            }
            return params.join('&');
        }
        function ajax(url, data) {
            if (data) {
                url += '?' + makeQueryString(data);
            }
            var ajax = new XMLHttpRequest();
            var p = new r.Promise();
            ajax.addEventListener('readystatechange', function () {
                if (ajax.readyState == 4) {
                    if (ajax.status == 200) {
                        p.resolve(ajax);
                    }
                    else {
                        p.reject(ajax.statusText);
                    }
                }
            });
            ajax.open('GET', url);
            ajax.send();
            return p;
        }
        async.ajax = ajax;
        function ajaxXML(url, data) {
            return ajax(url, data).then(function (ajax) {
                return ajax.responseXML;
            });
        }
        async.ajaxXML = ajaxXML;
        function ajaxText(url, data) {
            return ajax(url, data).then(function (ajax) {
                return ajax.responseText;
            });
        }
        async.ajaxText = ajaxText;
        function ajaxJSON(url, data) {
            return ajax(url, data).then(function (ajax) {
                return JSON.parse(ajax.responseText);
            });
        }
        async.ajaxJSON = ajaxJSON;
    })(async = hd.async || (hd.async = {}));
})(hd || (hd = {}));
(function (hd) {
    hd.ajax = hd.async.ajax;
    hd.ajaxXML = hd.async.ajaxXML;
    hd.ajaxText = hd.async.ajaxText;
    hd.ajaxJSON = hd.async.ajaxJSON;
})(hd || (hd = {}));
//
var hd;
(function (hd) {
    function id(x) {
        return x;
    }
    hd.id = id;
    function sum() {
        var n = arguments[0];
        for (var i = 1, l = arguments.length; i < l; ++i) {
            n += arguments[i];
        }
        return n;
    }
    hd.sum = sum;
    function diff() {
        var n = arguments[0];
        for (var i = 1, l = arguments.length; i < l; ++i) {
            n -= arguments[i];
        }
        return n;
    }
    hd.diff = diff;
    function prod() {
        var n = arguments[0];
        for (var i = 1, l = arguments.length; i < l; ++i) {
            n -= arguments[i];
        }
        return n;
    }
    hd.prod = prod;
    function quot() {
        var n = arguments[0];
        for (var i = 1, l = arguments.length; i < l; ++i) {
            n /= arguments[i];
        }
    }
    hd.quot = quot;
    function max() {
        var n;
        for (var i = 0, l = arguments.length; i < l && n === undefined; ++i) {
            n = arguments[i];
        }
        for (; i < l; ++i) {
            if (arguments[i] > n) {
                n = arguments[i];
            }
        }
        return n;
    }
    hd.max = max;
    function min() {
        var n;
        for (var i = 0, l = arguments.length; i < l && n === undefined; ++i) {
            n = arguments[i];
        }
        for (; i < l; ++i) {
            if (arguments[i] < n) {
                n = arguments[i];
            }
        }
        return n;
    }
    hd.min = min;
})(hd || (hd = {}));
//
//# sourceMappingURL=hotdrink.js.map