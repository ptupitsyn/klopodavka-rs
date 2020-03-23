(function () {
    'use strict';

    function wasm_bindgen_initialize( memory, table, alloc, free ) {
            var Module = {};
            Module.web_malloc = alloc;
            Module.web_free = free;
            Module.web_table = table;
            Object.defineProperty( Module, "HEAP8", {
                get: function() { return new Int8Array( memory.buffer ); }
            });
            Object.defineProperty( Module, "HEAP16", {
                get: function() { return new Int16Array( memory.buffer ); }
            });
            Object.defineProperty( Module, "HEAP32", {
                get: function() { return new Int32Array( memory.buffer ); }
            });
            Object.defineProperty( Module, "HEAPU8", {
                get: function() { return new Uint8Array( memory.buffer ); }
            });
            Object.defineProperty( Module, "HEAPU16", {
                get: function() { return new Uint16Array( memory.buffer ); }
            });
            Object.defineProperty( Module, "HEAPU32", {
                get: function() { return new Uint32Array( memory.buffer ); }
            });
            Object.defineProperty( Module, "HEAPF32", {
                get: function() { return new Float32Array( memory.buffer ); }
            });
            Object.defineProperty( Module, "HEAPF64", {
                get: function() { return new Float64Array( memory.buffer ); }
            });
            return Module;
        }

    function __cargo_web_snippet_ab05f53189dacccf2d365ad26daa407d4f7abea9(Module, $0, $1) { $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return ($1).value;})()); }

    function __cargo_web_snippet_b06dde4acf09433b5190a4b001259fe5d4abcbc2(Module, $0, $1) { $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return ($1).success;})()); }

    function __cargo_web_snippet_84339b1bf72a580059a6e0ff9499e53759aef5b9(Module, $0) { var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof MouseEvent && o.type === "click"); }

    function __cargo_web_snippet_e854289309e564012996fbb70e7c19bf4e6a8866(Module, $0) { var r = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (r instanceof DOMException) && (r.name === "NamespaceError"); }

    function __cargo_web_snippet_0e54fd9c163fcf648ce0a395fde4500fd167a40b(Module, $0) { var r = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (r instanceof DOMException) && (r.name === "InvalidCharacterError"); }

    function __cargo_web_snippet_7c8dfab835dc8a552cd9d67f27d26624590e052c(Module, $0) { var r = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (r instanceof DOMException) && (r.name === "SyntaxError"); }

    function __cargo_web_snippet_de2896a7ccf316486788a4d0bc433c25d2f1a12b(Module, $0) { var r = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (r instanceof DOMException) && (r.name === "NotFoundError"); }

    function __cargo_web_snippet_c023351d5bff43ef3dd317b499821cd4e71492f0(Module, $0) { var r = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (r instanceof DOMException) && (r.name === "HierarchyRequestError"); }

    function __cargo_web_snippet_690311d2f9134ac0983620c38a9e6460d4165607(Module, $0, $1) { $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return ($1).nextSibling;})()); }

    function __cargo_web_snippet_c26ddf75f581148e029dfcd95c037bb50d502e43(Module, $0, $1) { $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).value=($1); }

    function __cargo_web_snippet_db12d53e9596e9bc7860a8231ec85044629926e7(Module, $0) { var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof HTMLTextAreaElement); }

    function __cargo_web_snippet_2908dbb08792df5e699e324eec3e29fd6a57c2c9(Module, $0) { var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof HTMLInputElement); }

    function __cargo_web_snippet_3c5e83d16a83fc7147ec91e2506438012952f55a(Module, $0) { var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof Element); }

    function __cargo_web_snippet_f6358c198ebcc61c9da370cca2679c0b8bc81a7b(Module, $0, $1) { $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).removeAttribute(($1)); }

    function __cargo_web_snippet_09675c7ed2827e045dc760aeac3d286437cfbe5e(Module, $0, $1, $2, $3) { $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);Module.STDWEB_PRIVATE.from_js($0, (function(){try{return {value:function(){return ($1).setAttribute(($2),($3));}(),success:true};}catch(error){return {error:error,success:false};}})()); }

    function __cargo_web_snippet_5f8f03c2f100be177db5a7d58ca6b8cbbeaa0c93(Module, $0, $1) { $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return ($1).namespaceURI;})()); }

    function __cargo_web_snippet_465ffdf4814bf5d08a3abdf8fe5e61a220b98c34(Module, $0) { var o = Module.STDWEB_PRIVATE.acquire_js_reference( $0 );return (o instanceof Node); }

    function __cargo_web_snippet_08a3b15e1358700ac92bc556f9e9b8af660fc2c7(Module, $0, $1) { $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).nodeValue=($1); }

    function __cargo_web_snippet_f03767d5868baf486b51c1e3988d0ce100e850ca(Module, $0, $1) { $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return ($1).lastChild;})()); }

    function __cargo_web_snippet_46518012593da937dd5f35c2fc1c5e1dcade260b(Module, $0, $1, $2, $3) { $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);Module.STDWEB_PRIVATE.from_js($0, (function(){try{return {value:function(){return ($1).insertBefore(($2),($3));}(),success:true};}catch(error){return {error:error,success:false};}})()); }

    function __cargo_web_snippet_cd41a77d0178ae27c833ef2950e5f1a48a1455c1(Module, $0, $1, $2) { $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);Module.STDWEB_PRIVATE.from_js($0, (function(){try{return {value:function(){return ($1).removeChild(($2));}(),success:true};}catch(error){return {error:error,success:false};}})()); }

    function __cargo_web_snippet_e741b9d9071097746386b2c2ec044a2bc73e688c(Module, $0, $1) { $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).appendChild(($1)); }

    function __cargo_web_snippet_99c4eefdc8d4cc724135163b8c8665a1f3de99e4(Module, $0, $1, $2, $3) { $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);Module.STDWEB_PRIVATE.from_js($0, (function(){var listener=($1);($2).addEventListener(($3),listener);return listener;})()); }

    function __cargo_web_snippet_f750c7bda400081b4d7209f43f9d59214d39f6ea(Module, $0, $1, $2) { $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);var listener=($0);($1).removeEventListener(($2),listener);listener.drop(); }

    function __cargo_web_snippet_85b9ecbdb8513465b790546acfd0cd530441b8a4(Module, $0) { $0 = Module.STDWEB_PRIVATE.to_js($0);($0).stopPropagation(); }

    function __cargo_web_snippet_afafe9a462a05084fec65cacc7d6598e145ff3e3(Module, $0, $1, $2) { $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);Module.STDWEB_PRIVATE.from_js($0, (function(){return ($1).createTextNode(($2));})()); }

    function __cargo_web_snippet_0c326292efc233420ade892f70743acb4a852fd7(Module, $0, $1, $2, $3) { $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);$3 = Module.STDWEB_PRIVATE.to_js($3);Module.STDWEB_PRIVATE.from_js($0, (function(){try{return {value:function(){return ($1).createElementNS(($2),($3));}(),success:true};}catch(error){return {error:error,success:false};}})()); }

    function __cargo_web_snippet_91749aeb589cd0f9b17cbc01b2872ba709817982(Module, $0, $1, $2) { $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);Module.STDWEB_PRIVATE.from_js($0, (function(){try{return {value:function(){return ($1).createElement(($2));}(),success:true};}catch(error){return {error:error,success:false};}})()); }

    function __cargo_web_snippet_6fcce0aae651e2d748e085ff1f800f87625ff8c8(Module, $0) { Module.STDWEB_PRIVATE.from_js($0, (function(){return document;})()); }

    function __cargo_web_snippet_e9638d6405ab65f78daf4a5af9c9de14ecf1e2ec(Module, $0) { $0 = Module.STDWEB_PRIVATE.to_js($0);Module.STDWEB_PRIVATE.unregister_raw_value(($0)); }

    function __cargo_web_snippet_0aced9e2351ced72f1ff99645a129132b16c0d3c(Module, $0) { var value = Module.STDWEB_PRIVATE.get_raw_value( $0 );return Module.STDWEB_PRIVATE.register_raw_value( value ); }

    function __cargo_web_snippet_a152e8d0e8fac5476f30c1d19e4ab217dbcba73d(Module, $0, $1, $2) { $1 = Module.STDWEB_PRIVATE.to_js($1);$2 = Module.STDWEB_PRIVATE.to_js($2);Module.STDWEB_PRIVATE.from_js($0, (function(){try{return {value:function(){return ($1).querySelector(($2));}(),success:true};}catch(error){return {error:error,success:false};}})()); }

    function __cargo_web_snippet_ff5103e6cc179d13b4c7a785bdce2708fd559fc0(Module, $0) { Module.STDWEB_PRIVATE.tmp = Module.STDWEB_PRIVATE.to_js( $0 ); }

    function __cargo_web_snippet_614a3dd2adb7e9eac4a0ec6e59d37f87e0521c3b(Module, $0, $1) { $1 = Module.STDWEB_PRIVATE.to_js($1);Module.STDWEB_PRIVATE.from_js($0, (function(){return ($1).error;})()); }

    function __cargo_web_snippet_80d6d56760c65e49b7be8b6b01c1ea861b046bf0(Module, $0) { Module.STDWEB_PRIVATE.decrement_refcount( $0 ); }

    function __cargo_web_snippet_9f22d4ca7bc938409787341b7db181f8dd41e6df(Module, $0) { Module.STDWEB_PRIVATE.increment_refcount( $0 ); }

    function __cargo_web_snippet_72fc447820458c720c68d0d8e078ede631edd723(Module, $0, $1, $2) { console.error( 'Panic location:', Module.STDWEB_PRIVATE.to_js_string( $0, $1 ) + ':' + $2 ); }

    function __cargo_web_snippet_97495987af1720d8a9a923fa4683a7b683e3acd6(Module, $0, $1) { console.error( 'Panic error message:', Module.STDWEB_PRIVATE.to_js_string( $0, $1 ) ); }

    function __cargo_web_snippet_dc2fd915bd92f9e9c6a3bd15174f1414eee3dbaf(Module) { console.error( 'Encountered a panic!' ); }

    function __cargo_web_snippet_1c30acb32a1994a07c75e804ae9855b43f191d63(Module) { Module.STDWEB_PRIVATE = {};   Module.STDWEB_PRIVATE.to_utf8 = function to_utf8( str, addr ) {     var HEAPU8 = Module.HEAPU8;     for( var i = 0; i < str.length; ++i ) {                                    var u = str.charCodeAt( i );          if( u >= 0xD800 && u <= 0xDFFF ) {             u = 0x10000 + ((u & 0x3FF) << 10) | (str.charCodeAt( ++i ) & 0x3FF);         }          if( u <= 0x7F ) {             HEAPU8[ addr++ ] = u;         } else if( u <= 0x7FF ) {             HEAPU8[ addr++ ] = 0xC0 | (u >> 6);             HEAPU8[ addr++ ] = 0x80 | (u & 63);         } else if( u <= 0xFFFF ) {             HEAPU8[ addr++ ] = 0xE0 | (u >> 12);             HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);             HEAPU8[ addr++ ] = 0x80 | (u & 63);         } else if( u <= 0x1FFFFF ) {             HEAPU8[ addr++ ] = 0xF0 | (u >> 18);             HEAPU8[ addr++ ] = 0x80 | ((u >> 12) & 63);             HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);             HEAPU8[ addr++ ] = 0x80 | (u & 63);         } else if( u <= 0x3FFFFFF ) {             HEAPU8[ addr++ ] = 0xF8 | (u >> 24);             HEAPU8[ addr++ ] = 0x80 | ((u >> 18) & 63);             HEAPU8[ addr++ ] = 0x80 | ((u >> 12) & 63);             HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);             HEAPU8[ addr++ ] = 0x80 | (u & 63);         } else {             HEAPU8[ addr++ ] = 0xFC | (u >> 30);             HEAPU8[ addr++ ] = 0x80 | ((u >> 24) & 63);             HEAPU8[ addr++ ] = 0x80 | ((u >> 18) & 63);             HEAPU8[ addr++ ] = 0x80 | ((u >> 12) & 63);             HEAPU8[ addr++ ] = 0x80 | ((u >> 6) & 63);             HEAPU8[ addr++ ] = 0x80 | (u & 63);         }     } };  Module.STDWEB_PRIVATE.noop = function() {}; Module.STDWEB_PRIVATE.to_js = function to_js( address ) {     var kind = Module.HEAPU8[ address + 12 ];     if( kind === 0 ) {         return undefined;     } else if( kind === 1 ) {         return null;     } else if( kind === 2 ) {         return Module.HEAP32[ address / 4 ];     } else if( kind === 3 ) {         return Module.HEAPF64[ address / 8 ];     } else if( kind === 4 ) {         var pointer = Module.HEAPU32[ address / 4 ];         var length = Module.HEAPU32[ (address + 4) / 4 ];         return Module.STDWEB_PRIVATE.to_js_string( pointer, length );     } else if( kind === 5 ) {         return false;     } else if( kind === 6 ) {         return true;     } else if( kind === 7 ) {         var pointer = Module.STDWEB_PRIVATE.arena + Module.HEAPU32[ address / 4 ];         var length = Module.HEAPU32[ (address + 4) / 4 ];         var output = [];         for( var i = 0; i < length; ++i ) {             output.push( Module.STDWEB_PRIVATE.to_js( pointer + i * 16 ) );         }         return output;     } else if( kind === 8 ) {         var arena = Module.STDWEB_PRIVATE.arena;         var value_array_pointer = arena + Module.HEAPU32[ address / 4 ];         var length = Module.HEAPU32[ (address + 4) / 4 ];         var key_array_pointer = arena + Module.HEAPU32[ (address + 8) / 4 ];         var output = {};         for( var i = 0; i < length; ++i ) {             var key_pointer = Module.HEAPU32[ (key_array_pointer + i * 8) / 4 ];             var key_length = Module.HEAPU32[ (key_array_pointer + 4 + i * 8) / 4 ];             var key = Module.STDWEB_PRIVATE.to_js_string( key_pointer, key_length );             var value = Module.STDWEB_PRIVATE.to_js( value_array_pointer + i * 16 );             output[ key ] = value;         }         return output;     } else if( kind === 9 ) {         return Module.STDWEB_PRIVATE.acquire_js_reference( Module.HEAP32[ address / 4 ] );     } else if( kind === 10 || kind === 12 || kind === 13 ) {         var adapter_pointer = Module.HEAPU32[ address / 4 ];         var pointer = Module.HEAPU32[ (address + 4) / 4 ];         var deallocator_pointer = Module.HEAPU32[ (address + 8) / 4 ];         var num_ongoing_calls = 0;         var drop_queued = false;         var output = function() {             if( pointer === 0 || drop_queued === true ) {                 if (kind === 10) {                     throw new ReferenceError( "Already dropped Rust function called!" );                 } else if (kind === 12) {                     throw new ReferenceError( "Already dropped FnMut function called!" );                 } else {                     throw new ReferenceError( "Already called or dropped FnOnce function called!" );                 }             }              var function_pointer = pointer;             if (kind === 13) {                 output.drop = Module.STDWEB_PRIVATE.noop;                 pointer = 0;             }              if (num_ongoing_calls !== 0) {                 if (kind === 12 || kind === 13) {                     throw new ReferenceError( "FnMut function called multiple times concurrently!" );                 }             }              var args = Module.STDWEB_PRIVATE.alloc( 16 );             Module.STDWEB_PRIVATE.serialize_array( args, arguments );              try {                 num_ongoing_calls += 1;                 Module.STDWEB_PRIVATE.dyncall( "vii", adapter_pointer, [function_pointer, args] );                 var result = Module.STDWEB_PRIVATE.tmp;                 Module.STDWEB_PRIVATE.tmp = null;             } finally {                 num_ongoing_calls -= 1;             }              if( drop_queued === true && num_ongoing_calls === 0 ) {                 output.drop();             }              return result;         };          output.drop = function() {             if (num_ongoing_calls !== 0) {                 drop_queued = true;                 return;             }              output.drop = Module.STDWEB_PRIVATE.noop;             var function_pointer = pointer;             pointer = 0;              if (function_pointer != 0) {                 Module.STDWEB_PRIVATE.dyncall( "vi", deallocator_pointer, [function_pointer] );             }         };          return output;     } else if( kind === 14 ) {         var pointer = Module.HEAPU32[ address / 4 ];         var length = Module.HEAPU32[ (address + 4) / 4 ];         var array_kind = Module.HEAPU32[ (address + 8) / 4 ];         var pointer_end = pointer + length;          switch( array_kind ) {             case 0:                 return Module.HEAPU8.subarray( pointer, pointer_end );             case 1:                 return Module.HEAP8.subarray( pointer, pointer_end );             case 2:                 return Module.HEAPU16.subarray( pointer, pointer_end );             case 3:                 return Module.HEAP16.subarray( pointer, pointer_end );             case 4:                 return Module.HEAPU32.subarray( pointer, pointer_end );             case 5:                 return Module.HEAP32.subarray( pointer, pointer_end );             case 6:                 return Module.HEAPF32.subarray( pointer, pointer_end );             case 7:                 return Module.HEAPF64.subarray( pointer, pointer_end );         }     } else if( kind === 15 ) {         return Module.STDWEB_PRIVATE.get_raw_value( Module.HEAPU32[ address / 4 ] );     } };  Module.STDWEB_PRIVATE.serialize_object = function serialize_object( address, value ) {     var keys = Object.keys( value );     var length = keys.length;     var key_array_pointer = Module.STDWEB_PRIVATE.alloc( length * 8 );     var value_array_pointer = Module.STDWEB_PRIVATE.alloc( length * 16 );     Module.HEAPU8[ address + 12 ] = 8;     Module.HEAPU32[ address / 4 ] = value_array_pointer;     Module.HEAPU32[ (address + 4) / 4 ] = length;     Module.HEAPU32[ (address + 8) / 4 ] = key_array_pointer;     for( var i = 0; i < length; ++i ) {         var key = keys[ i ];         var key_address = key_array_pointer + i * 8;         Module.STDWEB_PRIVATE.to_utf8_string( key_address, key );          Module.STDWEB_PRIVATE.from_js( value_array_pointer + i * 16, value[ key ] );     } };  Module.STDWEB_PRIVATE.serialize_array = function serialize_array( address, value ) {     var length = value.length;     var pointer = Module.STDWEB_PRIVATE.alloc( length * 16 );     Module.HEAPU8[ address + 12 ] = 7;     Module.HEAPU32[ address / 4 ] = pointer;     Module.HEAPU32[ (address + 4) / 4 ] = length;     for( var i = 0; i < length; ++i ) {         Module.STDWEB_PRIVATE.from_js( pointer + i * 16, value[ i ] );     } };   var cachedEncoder = ( typeof TextEncoder === "function"     ? new TextEncoder( "utf-8" )          : ( typeof util === "object" && util && typeof util.TextEncoder === "function"         ? new util.TextEncoder( "utf-8" )                  : null ) );  if ( cachedEncoder != null ) {     Module.STDWEB_PRIVATE.to_utf8_string = function to_utf8_string( address, value ) {         var buffer = cachedEncoder.encode( value );         var length = buffer.length;         var pointer = 0;          if ( length > 0 ) {             pointer = Module.STDWEB_PRIVATE.alloc( length );             Module.HEAPU8.set( buffer, pointer );         }          Module.HEAPU32[ address / 4 ] = pointer;         Module.HEAPU32[ (address + 4) / 4 ] = length;     };  } else {     Module.STDWEB_PRIVATE.to_utf8_string = function to_utf8_string( address, value ) {         var length = Module.STDWEB_PRIVATE.utf8_len( value );         var pointer = 0;          if ( length > 0 ) {             pointer = Module.STDWEB_PRIVATE.alloc( length );             Module.STDWEB_PRIVATE.to_utf8( value, pointer );         }          Module.HEAPU32[ address / 4 ] = pointer;         Module.HEAPU32[ (address + 4) / 4 ] = length;     }; }  Module.STDWEB_PRIVATE.from_js = function from_js( address, value ) {     var kind = Object.prototype.toString.call( value );     if( kind === "[object String]" ) {         Module.HEAPU8[ address + 12 ] = 4;         Module.STDWEB_PRIVATE.to_utf8_string( address, value );     } else if( kind === "[object Number]" ) {         if( value === (value|0) ) {             Module.HEAPU8[ address + 12 ] = 2;             Module.HEAP32[ address / 4 ] = value;         } else {             Module.HEAPU8[ address + 12 ] = 3;             Module.HEAPF64[ address / 8 ] = value;         }     } else if( value === null ) {         Module.HEAPU8[ address + 12 ] = 1;     } else if( value === undefined ) {         Module.HEAPU8[ address + 12 ] = 0;     } else if( value === false ) {         Module.HEAPU8[ address + 12 ] = 5;     } else if( value === true ) {         Module.HEAPU8[ address + 12 ] = 6;     } else if( kind === "[object Symbol]" ) {         var id = Module.STDWEB_PRIVATE.register_raw_value( value );         Module.HEAPU8[ address + 12 ] = 15;         Module.HEAP32[ address / 4 ] = id;     } else {         var refid = Module.STDWEB_PRIVATE.acquire_rust_reference( value );         Module.HEAPU8[ address + 12 ] = 9;         Module.HEAP32[ address / 4 ] = refid;     } };   var cachedDecoder = ( typeof TextDecoder === "function"     ? new TextDecoder( "utf-8" )          : ( typeof util === "object" && util && typeof util.TextDecoder === "function"         ? new util.TextDecoder( "utf-8" )                  : null ) );  if ( cachedDecoder != null ) {     Module.STDWEB_PRIVATE.to_js_string = function to_js_string( index, length ) {         return cachedDecoder.decode( Module.HEAPU8.subarray( index, index + length ) );     };  } else {               Module.STDWEB_PRIVATE.to_js_string = function to_js_string( index, length ) {         var HEAPU8 = Module.HEAPU8;         index = index|0;         length = length|0;         var end = (index|0) + (length|0);         var output = "";         while( index < end ) {             var x = HEAPU8[ index++ ];             if( x < 128 ) {                 output += String.fromCharCode( x );                 continue;             }             var init = (x & (0x7F >> 2));             var y = 0;             if( index < end ) {                 y = HEAPU8[ index++ ];             }             var ch = (init << 6) | (y & 63);             if( x >= 0xE0 ) {                 var z = 0;                 if( index < end ) {                     z = HEAPU8[ index++ ];                 }                 var y_z = ((y & 63) << 6) | (z & 63);                 ch = init << 12 | y_z;                 if( x >= 0xF0 ) {                     var w = 0;                     if( index < end ) {                         w = HEAPU8[ index++ ];                     }                     ch = (init & 7) << 18 | ((y_z << 6) | (w & 63));                      output += String.fromCharCode( 0xD7C0 + (ch >> 10) );                     ch = 0xDC00 + (ch & 0x3FF);                 }             }             output += String.fromCharCode( ch );             continue;         }         return output;     }; }  Module.STDWEB_PRIVATE.id_to_ref_map = {}; Module.STDWEB_PRIVATE.id_to_refcount_map = {}; Module.STDWEB_PRIVATE.ref_to_id_map = new WeakMap();  Module.STDWEB_PRIVATE.ref_to_id_map_fallback = new Map(); Module.STDWEB_PRIVATE.last_refid = 1;  Module.STDWEB_PRIVATE.id_to_raw_value_map = {}; Module.STDWEB_PRIVATE.last_raw_value_id = 1;  Module.STDWEB_PRIVATE.acquire_rust_reference = function( reference ) {     if( reference === undefined || reference === null ) {         return 0;     }      var id_to_refcount_map = Module.STDWEB_PRIVATE.id_to_refcount_map;     var id_to_ref_map = Module.STDWEB_PRIVATE.id_to_ref_map;     var ref_to_id_map = Module.STDWEB_PRIVATE.ref_to_id_map;     var ref_to_id_map_fallback = Module.STDWEB_PRIVATE.ref_to_id_map_fallback;      var refid = ref_to_id_map.get( reference );     if( refid === undefined ) {         refid = ref_to_id_map_fallback.get( reference );     }     if( refid === undefined ) {         refid = Module.STDWEB_PRIVATE.last_refid++;         try {             ref_to_id_map.set( reference, refid );         } catch (e) {             ref_to_id_map_fallback.set( reference, refid );         }     }      if( refid in id_to_ref_map ) {         id_to_refcount_map[ refid ]++;     } else {         id_to_ref_map[ refid ] = reference;         id_to_refcount_map[ refid ] = 1;     }      return refid; };  Module.STDWEB_PRIVATE.acquire_js_reference = function( refid ) {     return Module.STDWEB_PRIVATE.id_to_ref_map[ refid ]; };  Module.STDWEB_PRIVATE.increment_refcount = function( refid ) {     Module.STDWEB_PRIVATE.id_to_refcount_map[ refid ]++; };  Module.STDWEB_PRIVATE.decrement_refcount = function( refid ) {     var id_to_refcount_map = Module.STDWEB_PRIVATE.id_to_refcount_map;     if( 0 == --id_to_refcount_map[ refid ] ) {         var id_to_ref_map = Module.STDWEB_PRIVATE.id_to_ref_map;         var ref_to_id_map_fallback = Module.STDWEB_PRIVATE.ref_to_id_map_fallback;         var reference = id_to_ref_map[ refid ];         delete id_to_ref_map[ refid ];         delete id_to_refcount_map[ refid ];         ref_to_id_map_fallback.delete(reference);     } };  Module.STDWEB_PRIVATE.register_raw_value = function( value ) {     var id = Module.STDWEB_PRIVATE.last_raw_value_id++;     Module.STDWEB_PRIVATE.id_to_raw_value_map[ id ] = value;     return id; };  Module.STDWEB_PRIVATE.unregister_raw_value = function( id ) {     delete Module.STDWEB_PRIVATE.id_to_raw_value_map[ id ]; };  Module.STDWEB_PRIVATE.get_raw_value = function( id ) {     return Module.STDWEB_PRIVATE.id_to_raw_value_map[ id ]; }; Module.STDWEB_PRIVATE.alloc = function alloc( size ) {     return Module.web_malloc( size ); };  Module.STDWEB_PRIVATE.dyncall = function( signature, ptr, args ) {     return Module.web_table.get( ptr ).apply( null, args ); };   Module.STDWEB_PRIVATE.utf8_len = function utf8_len( str ) {     var len = 0;     for( var i = 0; i < str.length; ++i ) {                           var u = str.charCodeAt( i );          if( u >= 0xD800 && u <= 0xDFFF ) {             u = 0x10000 + ((u & 0x3FF) << 10) | (str.charCodeAt( ++i ) & 0x3FF);         }          if( u <= 0x7F ) {             ++len;         } else if( u <= 0x7FF ) {             len += 2;         } else if( u <= 0xFFFF ) {             len += 3;         } else if( u <= 0x1FFFFF ) {             len += 4;         } else if( u <= 0x3FFFFFF ) {             len += 5;         } else {             len += 6;         }     }     return len; };  Module.STDWEB_PRIVATE.prepare_any_arg = function( value ) {     var arg = Module.STDWEB_PRIVATE.alloc( 16 );     Module.STDWEB_PRIVATE.from_js( arg, value );     return arg; };  Module.STDWEB_PRIVATE.acquire_tmp = function( dummy ) {     var value = Module.STDWEB_PRIVATE.tmp;     Module.STDWEB_PRIVATE.tmp = null;     return value; };  }

    function __cargo_web_snippet_a1f43b583e011a9bbeae64030b81f677e6c29005(Module, $0, $1) { $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).checked=($1); }

    function __cargo_web_snippet_0da47658267a7497de743e1b0892f992ba6ca6ef(Module, $0, $1) { $0 = Module.STDWEB_PRIVATE.to_js($0);$1 = Module.STDWEB_PRIVATE.to_js($1);($0).type=($1); }

    let wasm;

    const heap = new Array(32).fill(undefined);

    heap.push(undefined, null, true, false);

    function getObject(idx) { return heap[idx]; }

    let heap_next = heap.length;

    function addHeapObject(obj) {
        if (heap_next === heap.length) heap.push(heap.length + 1);
        const idx = heap_next;
        heap_next = heap[idx];

        heap[idx] = obj;
        return idx;
    }

    function dropObject(idx) {
        if (idx < 36) return;
        heap[idx] = heap_next;
        heap_next = idx;
    }

    function takeObject(idx) {
        const ret = getObject(idx);
        dropObject(idx);
        return ret;
    }

    let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

    cachedTextDecoder.decode();

    let cachegetUint8Memory0 = null;
    function getUint8Memory0() {
        if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory0;
    }

    function getStringFromWasm0(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
    }

    function makeClosure(arg0, arg1, dtor, f) {
        const state = { a: arg0, b: arg1, cnt: 1 };
        const real = (...args) => {
            // First up with a closure we increment the internal reference
            // count. This ensures that the Rust closure environment won't
            // be deallocated while we're invoking it.
            state.cnt++;
            try {
                return f(state.a, state.b, ...args);
            } finally {
                if (--state.cnt === 0) {
                    wasm.__wbindgen_export_0.get(dtor)(state.a, state.b);
                    state.a = 0;
                }
            }
        };
        real.original = state;
        return real;
    }
    function __wbg_adapter_14(arg0, arg1, arg2) {
        var ret = wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hb1285f2ffed9cd5f(arg0, arg1, arg2);
        return ret;
    }

    function __wbg_adapter_17(arg0, arg1, arg2, arg3) {
        wasm._dyn_core__ops__function__Fn__A__B___Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h76ff34ac8cccb530(arg0, arg1, arg2, arg3);
    }

    /**
    */
    function run_app() {
        wasm.run_app();
    }

    async function load(module, imports) {
        if (typeof Response === 'function' && module instanceof Response) {

            if (typeof WebAssembly.instantiateStreaming === 'function') {
                try {
                    return await WebAssembly.instantiateStreaming(module, imports);

                } catch (e) {
                    if (module.headers.get('Content-Type') != 'application/wasm') {
                        console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                    } else {
                        throw e;
                    }
                }
            }

            const bytes = await module.arrayBuffer();
            return await WebAssembly.instantiate(bytes, imports);

        } else {

            const instance = await WebAssembly.instantiate(module, imports);

            if (instance instanceof WebAssembly.Instance) {
                return { instance, module };

            } else {
                return instance;
            }
        }
    }

    async function init(input) {
        if (typeof input === 'undefined') {
            input = (document.currentScript && document.currentScript.src || new URL('bundle.js', document.baseURI).href).replace(/\.js$/, '_bg.wasm');
        }
        const imports = {};
        imports.wbg = {};
        imports.wbg.__wbg_cargowebsnippet80d6d56760c65e49b7be8b6b01c1ea861b046bf0_5a8953894b8affd6 = function(arg0, arg1) {
            __cargo_web_snippet_80d6d56760c65e49b7be8b6b01c1ea861b046bf0(takeObject(arg0), arg1);
        };
        imports.wbg.__wbg_cargowebsnippeta152e8d0e8fac5476f30c1d19e4ab217dbcba73d_d3aa4336afb90213 = function(arg0, arg1, arg2, arg3) {
            __cargo_web_snippet_a152e8d0e8fac5476f30c1d19e4ab217dbcba73d(takeObject(arg0), arg1, arg2, arg3);
        };
        imports.wbg.__wbg_cargowebsnippetb06dde4acf09433b5190a4b001259fe5d4abcbc2_d346638ea92aac60 = function(arg0, arg1, arg2) {
            __cargo_web_snippet_b06dde4acf09433b5190a4b001259fe5d4abcbc2(takeObject(arg0), arg1, arg2);
        };
        imports.wbg.__wbg_cargowebsnippet614a3dd2adb7e9eac4a0ec6e59d37f87e0521c3b_61a037ef81d9af77 = function(arg0, arg1, arg2) {
            __cargo_web_snippet_614a3dd2adb7e9eac4a0ec6e59d37f87e0521c3b(takeObject(arg0), arg1, arg2);
        };
        imports.wbg.__wbg_cargowebsnippetab05f53189dacccf2d365ad26daa407d4f7abea9_641d6b20c73343b4 = function(arg0, arg1, arg2) {
            __cargo_web_snippet_ab05f53189dacccf2d365ad26daa407d4f7abea9(takeObject(arg0), arg1, arg2);
        };
        imports.wbg.__wbg_cargowebsnippet3c5e83d16a83fc7147ec91e2506438012952f55a_23154670c635c22b = function(arg0, arg1) {
            var ret = __cargo_web_snippet_3c5e83d16a83fc7147ec91e2506438012952f55a(takeObject(arg0), arg1);
            return ret;
        };
        imports.wbg.__wbg_cargowebsnippet7c8dfab835dc8a552cd9d67f27d26624590e052c_0fd666abcb082554 = function(arg0, arg1) {
            var ret = __cargo_web_snippet_7c8dfab835dc8a552cd9d67f27d26624590e052c(takeObject(arg0), arg1);
            return ret;
        };
        imports.wbg.__wbg_cargowebsnippete9638d6405ab65f78daf4a5af9c9de14ecf1e2ec_ad1e81894f802539 = function(arg0, arg1) {
            __cargo_web_snippet_e9638d6405ab65f78daf4a5af9c9de14ecf1e2ec(takeObject(arg0), arg1);
        };
        imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
            var ret = getObject(arg0);
            return addHeapObject(ret);
        };
        imports.wbg.__wbindgen_memory = function() {
            var ret = wasm.memory;
            return addHeapObject(ret);
        };
        imports.wbg.__wbindgen_function_table = function() {
            var ret = wasm.__wbindgen_export_0;
            return addHeapObject(ret);
        };
        imports.wbg.__wbg_wasmbindgeninitialize_c1c4df6b494511ad = function(arg0, arg1, arg2, arg3) {
            var ret = wasm_bindgen_initialize(takeObject(arg0), takeObject(arg1), getObject(arg2), getObject(arg3));
            return addHeapObject(ret);
        };
        imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
            takeObject(arg0);
        };
        imports.wbg.__wbindgen_cb_forget = function(arg0) {
            takeObject(arg0);
        };
        imports.wbg.__wbg_cargowebsnippet1c30acb32a1994a07c75e804ae9855b43f191d63_6d353463ef525961 = function(arg0) {
            __cargo_web_snippet_1c30acb32a1994a07c75e804ae9855b43f191d63(takeObject(arg0));
        };
        imports.wbg.__wbg_cargowebsnippetdc2fd915bd92f9e9c6a3bd15174f1414eee3dbaf_ce5c721cab10d020 = function(arg0) {
            __cargo_web_snippet_dc2fd915bd92f9e9c6a3bd15174f1414eee3dbaf(takeObject(arg0));
        };
        imports.wbg.__wbg_cargowebsnippet97495987af1720d8a9a923fa4683a7b683e3acd6_a438202dc16f44c0 = function(arg0, arg1, arg2) {
            __cargo_web_snippet_97495987af1720d8a9a923fa4683a7b683e3acd6(takeObject(arg0), arg1, arg2);
        };
        imports.wbg.__wbg_cargowebsnippet72fc447820458c720c68d0d8e078ede631edd723_ece3da0a4474dbeb = function(arg0, arg1, arg2, arg3) {
            __cargo_web_snippet_72fc447820458c720c68d0d8e078ede631edd723(takeObject(arg0), arg1, arg2, arg3);
        };
        imports.wbg.__wbg_cargowebsnippet6fcce0aae651e2d748e085ff1f800f87625ff8c8_21ca3c3552146790 = function(arg0, arg1) {
            __cargo_web_snippet_6fcce0aae651e2d748e085ff1f800f87625ff8c8(takeObject(arg0), arg1);
        };
        imports.wbg.__wbg_cargowebsnippet0e54fd9c163fcf648ce0a395fde4500fd167a40b_79ac4825f71161f4 = function(arg0, arg1) {
            var ret = __cargo_web_snippet_0e54fd9c163fcf648ce0a395fde4500fd167a40b(takeObject(arg0), arg1);
            return ret;
        };
        imports.wbg.__wbg_cargowebsnippetafafe9a462a05084fec65cacc7d6598e145ff3e3_d374efb355d898fd = function(arg0, arg1, arg2, arg3) {
            __cargo_web_snippet_afafe9a462a05084fec65cacc7d6598e145ff3e3(takeObject(arg0), arg1, arg2, arg3);
        };
        imports.wbg.__wbg_cargowebsnippet0aced9e2351ced72f1ff99645a129132b16c0d3c_13e902b8d846fb01 = function(arg0, arg1) {
            var ret = __cargo_web_snippet_0aced9e2351ced72f1ff99645a129132b16c0d3c(takeObject(arg0), arg1);
            return ret;
        };
        imports.wbg.__wbg_cargowebsnippet9f22d4ca7bc938409787341b7db181f8dd41e6df_f184afed978d4a95 = function(arg0, arg1) {
            __cargo_web_snippet_9f22d4ca7bc938409787341b7db181f8dd41e6df(takeObject(arg0), arg1);
        };
        imports.wbg.__wbg_cargowebsnippetde2896a7ccf316486788a4d0bc433c25d2f1a12b_cfdc90cf53c4d5da = function(arg0, arg1) {
            var ret = __cargo_web_snippet_de2896a7ccf316486788a4d0bc433c25d2f1a12b(takeObject(arg0), arg1);
            return ret;
        };
        imports.wbg.__wbg_cargowebsnippetc023351d5bff43ef3dd317b499821cd4e71492f0_a6f092f3cdeb4fef = function(arg0, arg1) {
            var ret = __cargo_web_snippet_c023351d5bff43ef3dd317b499821cd4e71492f0(takeObject(arg0), arg1);
            return ret;
        };
        imports.wbg.__wbindgen_throw = function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        };
        imports.wbg.__wbg_cargowebsnippet84339b1bf72a580059a6e0ff9499e53759aef5b9_4e8ea0b89beeafa8 = function(arg0, arg1) {
            var ret = __cargo_web_snippet_84339b1bf72a580059a6e0ff9499e53759aef5b9(takeObject(arg0), arg1);
            return ret;
        };
        imports.wbg.__wbg_cargowebsnippet85b9ecbdb8513465b790546acfd0cd530441b8a4_c156ae51cab56530 = function(arg0, arg1) {
            __cargo_web_snippet_85b9ecbdb8513465b790546acfd0cd530441b8a4(takeObject(arg0), arg1);
        };
        imports.wbg.__wbg_cargowebsnippetff5103e6cc179d13b4c7a785bdce2708fd559fc0_c2c7bf9cb65f32b6 = function(arg0, arg1) {
            __cargo_web_snippet_ff5103e6cc179d13b4c7a785bdce2708fd559fc0(takeObject(arg0), arg1);
        };
        imports.wbg.__wbg_cargowebsnippetf03767d5868baf486b51c1e3988d0ce100e850ca_bc1dd209260bf552 = function(arg0, arg1, arg2) {
            __cargo_web_snippet_f03767d5868baf486b51c1e3988d0ce100e850ca(takeObject(arg0), arg1, arg2);
        };
        imports.wbg.__wbg_cargowebsnippet465ffdf4814bf5d08a3abdf8fe5e61a220b98c34_54cd4762544a0ed8 = function(arg0, arg1) {
            var ret = __cargo_web_snippet_465ffdf4814bf5d08a3abdf8fe5e61a220b98c34(takeObject(arg0), arg1);
            return ret;
        };
        imports.wbg.__wbg_cargowebsnippete741b9d9071097746386b2c2ec044a2bc73e688c_1d0e2ab1e286c949 = function(arg0, arg1, arg2) {
            __cargo_web_snippet_e741b9d9071097746386b2c2ec044a2bc73e688c(takeObject(arg0), arg1, arg2);
        };
        imports.wbg.__wbg_cargowebsnippet690311d2f9134ac0983620c38a9e6460d4165607_a4cb2a84cd08d937 = function(arg0, arg1, arg2) {
            __cargo_web_snippet_690311d2f9134ac0983620c38a9e6460d4165607(takeObject(arg0), arg1, arg2);
        };
        imports.wbg.__wbg_cargowebsnippetcd41a77d0178ae27c833ef2950e5f1a48a1455c1_f91f2bcf70515796 = function(arg0, arg1, arg2, arg3) {
            __cargo_web_snippet_cd41a77d0178ae27c833ef2950e5f1a48a1455c1(takeObject(arg0), arg1, arg2, arg3);
        };
        imports.wbg.__wbg_cargowebsnippet46518012593da937dd5f35c2fc1c5e1dcade260b_ea53969a3495cf2c = function(arg0, arg1, arg2, arg3, arg4) {
            __cargo_web_snippet_46518012593da937dd5f35c2fc1c5e1dcade260b(takeObject(arg0), arg1, arg2, arg3, arg4);
        };
        imports.wbg.__wbg_cargowebsnippet09675c7ed2827e045dc760aeac3d286437cfbe5e_f9f31cc69edf0c6a = function(arg0, arg1, arg2, arg3, arg4) {
            __cargo_web_snippet_09675c7ed2827e045dc760aeac3d286437cfbe5e(takeObject(arg0), arg1, arg2, arg3, arg4);
        };
        imports.wbg.__wbg_cargowebsnippet08a3b15e1358700ac92bc556f9e9b8af660fc2c7_9e8cd6528929926d = function(arg0, arg1, arg2) {
            __cargo_web_snippet_08a3b15e1358700ac92bc556f9e9b8af660fc2c7(takeObject(arg0), arg1, arg2);
        };
        imports.wbg.__wbg_cargowebsnippet5f8f03c2f100be177db5a7d58ca6b8cbbeaa0c93_afb20faf3d98c143 = function(arg0, arg1, arg2) {
            __cargo_web_snippet_5f8f03c2f100be177db5a7d58ca6b8cbbeaa0c93(takeObject(arg0), arg1, arg2);
        };
        imports.wbg.__wbg_cargowebsnippet91749aeb589cd0f9b17cbc01b2872ba709817982_594ed33c557be4ae = function(arg0, arg1, arg2, arg3) {
            __cargo_web_snippet_91749aeb589cd0f9b17cbc01b2872ba709817982(takeObject(arg0), arg1, arg2, arg3);
        };
        imports.wbg.__wbg_cargowebsnippet0c326292efc233420ade892f70743acb4a852fd7_828e3eda227fc4f5 = function(arg0, arg1, arg2, arg3, arg4) {
            __cargo_web_snippet_0c326292efc233420ade892f70743acb4a852fd7(takeObject(arg0), arg1, arg2, arg3, arg4);
        };
        imports.wbg.__wbg_cargowebsnippete854289309e564012996fbb70e7c19bf4e6a8866_cf4f9c19f2bbb277 = function(arg0, arg1) {
            var ret = __cargo_web_snippet_e854289309e564012996fbb70e7c19bf4e6a8866(takeObject(arg0), arg1);
            return ret;
        };
        imports.wbg.__wbg_cargowebsnippet2908dbb08792df5e699e324eec3e29fd6a57c2c9_1aba12964286db2a = function(arg0, arg1) {
            var ret = __cargo_web_snippet_2908dbb08792df5e699e324eec3e29fd6a57c2c9(takeObject(arg0), arg1);
            return ret;
        };
        imports.wbg.__wbg_cargowebsnippetf6358c198ebcc61c9da370cca2679c0b8bc81a7b_6b46ebed0e760a39 = function(arg0, arg1, arg2) {
            __cargo_web_snippet_f6358c198ebcc61c9da370cca2679c0b8bc81a7b(takeObject(arg0), arg1, arg2);
        };
        imports.wbg.__wbg_cargowebsnippetdb12d53e9596e9bc7860a8231ec85044629926e7_78e4e5f2c2ce9f1f = function(arg0, arg1) {
            var ret = __cargo_web_snippet_db12d53e9596e9bc7860a8231ec85044629926e7(takeObject(arg0), arg1);
            return ret;
        };
        imports.wbg.__wbg_cargowebsnippetc26ddf75f581148e029dfcd95c037bb50d502e43_f494f4a63cf92998 = function(arg0, arg1, arg2) {
            __cargo_web_snippet_c26ddf75f581148e029dfcd95c037bb50d502e43(takeObject(arg0), arg1, arg2);
        };
        imports.wbg.__wbg_cargowebsnippet0da47658267a7497de743e1b0892f992ba6ca6ef_037901b7aa791b5a = function(arg0, arg1, arg2) {
            __cargo_web_snippet_0da47658267a7497de743e1b0892f992ba6ca6ef(takeObject(arg0), arg1, arg2);
        };
        imports.wbg.__wbg_cargowebsnippeta1f43b583e011a9bbeae64030b81f677e6c29005_a889c7efde32f5a2 = function(arg0, arg1, arg2) {
            __cargo_web_snippet_a1f43b583e011a9bbeae64030b81f677e6c29005(takeObject(arg0), arg1, arg2);
        };
        imports.wbg.__wbg_cargowebsnippetf750c7bda400081b4d7209f43f9d59214d39f6ea_c158729eaadb8dbe = function(arg0, arg1, arg2, arg3) {
            __cargo_web_snippet_f750c7bda400081b4d7209f43f9d59214d39f6ea(takeObject(arg0), arg1, arg2, arg3);
        };
        imports.wbg.__wbg_cargowebsnippet99c4eefdc8d4cc724135163b8c8665a1f3de99e4_14389f68322b60f1 = function(arg0, arg1, arg2, arg3, arg4) {
            __cargo_web_snippet_99c4eefdc8d4cc724135163b8c8665a1f3de99e4(takeObject(arg0), arg1, arg2, arg3, arg4);
        };
        imports.wbg.__wbindgen_closure_wrapper294 = function(arg0, arg1, arg2) {
            var ret = makeClosure(arg0, arg1, 24, __wbg_adapter_14);
            return addHeapObject(ret);
        };
        imports.wbg.__wbindgen_closure_wrapper295 = function(arg0, arg1, arg2) {
            var ret = makeClosure(arg0, arg1, 27, __wbg_adapter_17);
            return addHeapObject(ret);
        };

        if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
            input = fetch(input);
        }

        const { instance, module } = await load(await input, imports);

        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;

        return wasm;
    }

    async function main() {
       await init('./pkg/klopodavka_yew_bg.wasm');
       run_app();
    }
    main();

}());
