# Project HIFLAMES: Building a Bridge to the Future (Part 5)

<a href="https://opencollective.com/medicalmasses-llc/projects/rumtk-v2" rel="Help finance the project!">
    <img src="https://opencollective.com/medicalmasses-llc/contribute/button@2x.png?color=blue" width=300 />
</a>

## Financial Contributors
<object type="image/svg+xml" data="https://opencollective.com/medicalmasses-llc/tiers/backers.svg?avatarHeight=36&width=600"></object>

## Articles in Series
* [Project HIFLAMES: Building a Bridge to the Future (Part 1)](./intro.md)
* [Project HIFLAMES: Building a Bridge to the Future (Part 2)](./methods.md)
* [Project HIFLAMES: Building a Bridge to the Future (Part 3)](./results1.md)
* [Project HIFLAMES: Building a Bridge to the Future (Part 4)](./results2.md)
* [Project HIFLAMES: Building a Bridge to the Future (Part 5)](./results3.md)

## Important Links
* OpenCollective: https://opencollective.com/medicalmasses-llc/projects/rumtk-v2
* Website: https://www.medicalmasses.com/
* GiHub Repository: https://github.com/MedicalMasses-L-L-C/rumtk

## Introduction

We have been optimizing the V2 parser quite extensively. Here's a few ideas that we keep cycling by:

* **SIMD:** The main optimization tool we have been interested in has been ensuring that our algorithms respect SIMD instructions.
* **Arena Allocations:** This is a memory management strategy I recently learned about from listening to [Casey Muratori's](https://caseymuratori.com/) interviews and podcasts.
* **Cache Alignment:** I have mentioned this at the beginning and I will repeat how important it is that data fits and flows into the CPU caches to maximize the real performance of the thinking rock.
* **Branch Optimizations:** Minimizing branch misses can help gain more performance and keep it reliable.
* **O Characteristics:** After everything else has been taken care of it is important to review the general scalability of the algorithm.

As you can see, a lot of the work is very intertwined. For example, fewer branches can often lead to fewer cache references; fewer cache references
can lead to fewer cache misses; fewer cache misses often leads to fewer stalls and fewer cpu cycles to achieve the same work.
Similarly, Arena allocations minimize the need to ask for more memory from the kernel. Once the memory is allocated, it is easier to 
divide it up as needed. The strategy relies on pointer arithmetic and its usefulness comes in the form of deferring deallocations until 
the program or unit of work. Furthermore, the allocated memory is guaranteed to be continuous and minimize memory fragmentation.
It is a superb strategy for the allocation and deallocation of many small objects. It is a terrible strategy if looking to have random 
allocations.

SIMD is an old topic most people do not think about when implementing algorithms. SIMD stands for `Single Instruction Multiple Data`.
It is a set of instructions that come in many flavors. CPU manufacturers such as Intel and AMD place a lot of effort and die capital towards these 
instructions. Basically, instead of grabbing one byte to do work during an iteration, you can grab 4, 8, 16, 32, or even 64 bytes at once and apply the work 
on that chunk. For our purposes, we can check if a separator byte is present in our `haystack` and even get its index in one step.
This further means that you can linearize an algorithm which helps the CPU predict the next chunk prefetch and thus have the next 
segment of data ready to search through immediately. Also, it is disrespectful to users if we do not maximize the utilization of 
CPU facilities they already paid for. The efficiency is so great that often you can create otherwise `Linear` algorithms that can 
outperform `Logarithmic` algorithms. With that said, we should not forget about the lessons in `Computer Science` for the higher levels
of abstraction of the algorithm.

## The Report
### Flamegraph

<img src="imgs/optimized_flamegraph2.png" alt="Mean Time [ms] Processing a 2MB Message" width="700px">
Mean Time [ms] Processing a 2MB Message

### CPU Statistics
```
# started on Tue Jul 14 18:20:08 2026


 Performance counter stats for '../target/release/rumtk-hl7-v2-parse':

         1,409,557      cache-references:u                                                    
           129,004      cache-misses:u                                                        
        20,611,789      cycles:u                                                              
        57,375,536      instructions:u                                                        
        12,602,580      branches:u                                                            
               706      faults:u                                                              
                 0      migrations:u                                                          

       0.014727593 seconds time elapsed

       0.004861000 seconds user
       0.008774000 seconds sys
```

### CPU Info and Cache Budget Report
```
# To display the perf.data header info, please use --header/--header-only options.
#
#
# Total Lost Samples: 0
#
# Samples: 42  of event 'cache-references:u'
# Event count (approx.): 1334528
#
# Children      Self  Command          Shared Object         Symbol                                                                                                                                                                                                                                                        
# ........  ........  ...............  ....................  ..............................................................................................................................................................................................................................................................
#
    40.15%    40.15%  rumtk-hl7-v2-pa  libc.so.6             [.] __memmove_avx_unaligned_erms
            |
            ---__memmove_avx_unaligned_erms

    18.59%    18.59%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] serde_json::ser::format_escaped_str::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>
            |
            ---serde_json::ser::format_escaped_str::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>

    11.97%    11.97%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Field>::from
            |
            ---<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Field>::from

    10.62%    10.62%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Segment>::from
            |
            ---<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Segment>::from

     5.01%     5.01%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <serde_json::ser::Compound<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter> as serde_core::ser::SerializeMap>::serialize_entry::<str, alloc::vec::Vec<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Component>>
            |
            ---<serde_json::ser::Compound<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter> as serde_core::ser::SerializeMap>::serialize_entry::<str, alloc::vec::Vec<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Component>>



# Samples: 40  of event 'cache-misses:u'
# Event count (approx.): 98949
#
# Children      Self  Command          Shared Object         Symbol                                                                                                               
# ........  ........  ...............  ....................  .....................................................................................................................
#
    20.70%    20.70%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Segment>::from
            |
            ---<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Segment>::from

    13.65%     0.00%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] _start
            |
            ---_start
               |          
                --11.47%--__libc_start_main@@GLIBC_2.34
                          main
                          std::rt::lang_start_internal

    12.22%    12.22%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] serde_json::ser::format_escaped_str::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>
            |
            ---serde_json::ser::format_escaped_str::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>

    12.15%    12.15%  rumtk-hl7-v2-pa  libc.so.6             [.] __memmove_avx_unaligned_erms
            |
            ---__memmove_avx_unaligned_erms

    11.47%    11.47%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] std::rt::lang_start_internal
            |
            ---_start
               __libc_start_main@@GLIBC_2.34
               main
               std::rt::lang_start_internal

    11.47%     0.00%  rumtk-hl7-v2-pa  libc.so.6             [.] __libc_start_main@@GLIBC_2.34
            |
            ---__libc_start_main@@GLIBC_2.34
               main
               std::rt::lang_start_internal

    11.47%     0.00%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] main
            |
            ---main
               std::rt::lang_start_internal

     9.48%     9.48%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Field>::from
            |
            ---<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Field>::from

     7.69%     7.69%  rumtk-hl7-v2-pa  libc.so.6             [.] __memset_avx2_unaligned_erms
            |
            ---__memset_avx2_unaligned_erms

     6.40%     6.40%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Message>::try_from_buffer
            |
            ---<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Message>::try_from_buffer



# Samples: 45  of event 'branches:u'
# Event count (approx.): 12320841
#
# Children      Self  Command          Shared Object         Symbol                                                                                                                                                                                                                                                        
# ........  ........  ...............  ....................  ..............................................................................................................................................................................................................................................................
#
    34.92%    34.92%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] serde_json::ser::format_escaped_str::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>
            |
            ---serde_json::ser::format_escaped_str::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>

    18.74%    18.74%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Segment>::from
            |
            ---<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Segment>::from

    10.05%    10.05%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] core::str::converts::from_utf8
            |
            ---core::str::converts::from_utf8

     9.01%     9.01%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Field>::from
            |
            ---<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Field>::from

     8.04%     8.04%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <serde_json::ser::Compound<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter> as serde_core::ser::SerializeMap>::serialize_entry::<str, alloc::vec::Vec<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Component>>
            |
            ---<serde_json::ser::Compound<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter> as serde_core::ser::SerializeMap>::serialize_entry::<str, alloc::vec::Vec<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Component>>

     5.35%     5.35%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <serde_json::ser::Compound<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter> as serde_core::ser::SerializeMap>::serialize_entry::<str, alloc::vec::Vec<core::option::Option<alloc::vec::Vec<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Field>>>>
            |
            ---<serde_json::ser::Compound<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter> as serde_core::ser::SerializeMap>::serialize_entry::<str, alloc::vec::Vec<core::option::Option<alloc::vec::Vec<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Field>>>>

     5.07%     5.07%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] _mi_page_malloc_zero
            |
            ---_mi_page_malloc_zero



# Samples: 31  of event 'branch-misses:u'
# Event count (approx.): 42983
#
# Children      Self  Command          Shared Object         Symbol                                                                                                
# ........  ........  ...............  ....................  ......................................................................................................
#
    25.85%    25.85%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Segment>::from
            |
            ---<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Segment>::from

    17.74%    17.74%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] core::str::converts::from_utf8
            |
            ---core::str::converts::from_utf8

    16.76%    16.76%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Field>::from
            |
            ---<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Field>::from

    10.06%    10.06%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] mi_bin
            |
            ---mi_bin

     7.48%     0.03%  rumtk-hl7-v2-pa  ld-linux-x86-64.so.2  [.] _dl_start
            |          
             --7.45%--_dl_start
                       |          
                        --7.45%--_dl_start_final (inlined)
                                  |          
                                   --7.03%--_dl_sysdep_start
                                             |          
                                              --6.75%--dl_main

     7.47%     0.00%  rumtk-hl7-v2-pa  ld-linux-x86-64.so.2  [.] _dl_start_user
            |
            ---_dl_start_user
               _dl_start
               |          
                --7.45%--_dl_start_final (inlined)
                          |          
                           --7.03%--_dl_sysdep_start
                                     |          
                                      --6.75%--dl_main

     7.45%     0.00%  rumtk-hl7-v2-pa  ld-linux-x86-64.so.2  [.] _dl_start_final (inlined)
            |
            ---_dl_start_final (inlined)

     7.03%     0.00%  rumtk-hl7-v2-pa  ld-linux-x86-64.so.2  [.] _dl_sysdep_start
            |
            ---_dl_sysdep_start
               |          
                --6.75%--dl_main

     6.75%     3.24%  rumtk-hl7-v2-pa  ld-linux-x86-64.so.2  [.] _dl_lookup_symbol_x
     6.75%     0.00%  rumtk-hl7-v2-pa  ld-linux-x86-64.so.2  [.] dl_main
            |
            ---dl_main

     6.17%     6.17%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Message>::try_from_buffer
            |
            ---<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Message>::try_from_buffer

     6.09%     6.09%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] serde_json::ser::format_escaped_str::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>
            |
            ---serde_json::ser::format_escaped_str::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>

     5.06%     5.06%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] _mi_arenas_page_unabandon
            |
            ---_mi_arenas_page_unabandon



# Samples: 49  of event 'cycles:u'
# Event count (approx.): 19148164
#
# Children      Self  Command          Shared Object              Symbol                                                                                                                                                                                                                                                        
# ........  ........  ...............  .........................  ..............................................................................................................................................................................................................................................................
#
    19.73%    19.73%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse         [.] <rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Segment>::from
            |
            ---<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Segment>::from

    17.84%    17.84%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse         [.] serde_json::ser::format_escaped_str::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>
            |
            ---serde_json::ser::format_escaped_str::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>

    11.94%    11.94%  rumtk-hl7-v2-pa  libc.so.6                  [.] __memmove_avx_unaligned_erms
            |
            ---__memmove_avx_unaligned_erms

    11.50%    11.50%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse         [.] <serde_json::ser::Compound<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter> as serde_core::ser::SerializeMap>::serialize_entry::<str, alloc::vec::Vec<core::option::Option<alloc::vec::Vec<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Field>>>>
            |
            ---<serde_json::ser::Compound<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter> as serde_core::ser::SerializeMap>::serialize_entry::<str, alloc::vec::Vec<core::option::Option<alloc::vec::Vec<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Field>>>>

     7.66%     7.66%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse         [.] <serde_json::ser::Compound<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter> as serde_core::ser::SerializeMap>::serialize_entry::<str, alloc::vec::Vec<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Component>>
            |
            ---<serde_json::ser::Compound<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter> as serde_core::ser::SerializeMap>::serialize_entry::<str, alloc::vec::Vec<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Component>>

     6.98%     6.98%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse         [.] <rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Component as serde_core::ser::Serialize>::serialize::<&mut serde_json::ser::Serializer<&mut alloc::vec::Vec<u8>>>
            |
            ---<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Component as serde_core::ser::Serialize>::serialize::<&mut serde_json::ser::Serializer<&mut alloc::vec::Vec<u8>>>

     5.89%     5.89%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse         [.] core::str::converts::from_utf8
            |
            ---core::str::converts::from_utf8



# Samples: 35  of event 'idle-cycles-frontend:u'
# Event count (approx.): 829462
#
# Children      Self  Command          Shared Object         Symbol                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             
# ........  ........  ...............  ....................  ...................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................
#
    11.67%    11.67%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] serde_json::ser::format_escaped_str::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>
            |
            ---serde_json::ser::format_escaped_str::<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter>

    11.20%    11.20%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <serde_json::ser::Compound<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter> as serde_core::ser::SerializeMap>::serialize_entry::<str, rumtk_core::buffers::rumbuffer::RUMBuffer>
            |
            ---<serde_json::ser::Compound<&mut alloc::vec::Vec<u8>, serde_json::ser::CompactFormatter> as serde_core::ser::SerializeMap>::serialize_entry::<str, rumtk_core::buffers::rumbuffer::RUMBuffer>

    11.14%    11.14%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Segment>::from
            |
            ---<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Segment>::from

     8.01%     8.01%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Field>::from
            |
            ---<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Field>::from

     7.98%     7.98%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] mi_bitmap_setN
            |
            ---mi_bitmap_setN

     7.59%     7.59%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <indexmap::map::IndexMap<u8, alloc::vec::Vec<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Segment>>>::get_mut::<u8>
            |
            ---<indexmap::map::IndexMap<u8, alloc::vec::Vec<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Segment>>>::get::<u8> (inlined)

     7.59%     0.00%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <indexmap::map::IndexMap<u8, alloc::vec::Vec<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Segment>>>::get::<u8> (inlined)
            |
            ---<indexmap::map::IndexMap<u8, alloc::vec::Vec<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Segment>>>::get::<u8> (inlined)

     7.56%     7.56%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] mi_page_free_list_extend
            |
            ---mi_page_free_list_extend

     6.99%     6.99%  rumtk-hl7-v2-pa  rumtk-hl7-v2-parse    [.] <rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Message>::try_from_buffer
            |
            ---<rumtk_hl7_v2::hl7_v2_parser::v2_parser::V2Message>::try_from_buffer



#
# (Tip: Use --symfs <dir>[,layout] if your symbol files are in non-standard locations.)
#
```

## Results
The optimized parser showed a mean parsing time of 11.6 ms with a standard deviation of 0.5 ms. The minimum parsing time was 10.6 ms and the maximum time was 13.4 ms.

The encoding to JSON step took at least about 42.13% of the execution time. Not shown here is that we simplified struct field naming to minimize the output JSON size.

Memory allocations took 44.08% of the execution time.

The CPU experienced 129,004 cache misses, 20,611,789 cycles, 12,602,580 branches, and 706 page faults. Overall, we have improved on the CPU metrics.

The flamegraph shows a 40/60 split between parsing and everything else (memory management and JSON serialization).

## Discussion
There are a few things that were implemented in this round of optimizations. We rewrote the `RUMBuffer` struct to remove the implicit `Vec` in the version of the type found in the
`bytes` crate. Now, the `bytes` crate is a superb crate. I only noticed that their strategy used a vector under the hood because optimizations were so good that the report 
began picking up on the lower level functions taking more and more proportion of the execution time. In addition, I was looking for an annoying source of memory allocations that 
did not seem to come from my algorithm. Our version can be optimized further since it is currently a 48 byte struct or 75% of a typical cache line... Stay tune for more.

Something else that was done was optimizing the output JSON serialization to minimize needed memory and speed up transfers. This optimization was mostly about minifying field names 
to make sure `Serde` serialized those in a smaller form. We also converted the empty field case to null (`None`) and deferred repeatedly calling the constructor. Along these lines, 
we also added a few minor optimizations to decision paths in which I could go through a default construction or simply skip and return a `V2Component` instance with the full pointer.

We added a new family of helper functions under the `cpu` module optimized towards `SIMD` with non-SIMD fallbacks. These functions take care from detecting if a byte is present in the 
`slice` to counting instances of the byte in sequence to finding the index of the byte in the sequence to replacing all instances of the byte with another byte. All of these functions 
use `AVX` instructions by default, although we rely on the Portable SIMD compiler feature present in `nightly` which should hypothetically allow us to recompile to other platforms. 

Next, we reviewed methods and inlined them with the `#[inline]` attribute. We set the compiler optimization levels to max including `LTO`. We repeatedly measured to ensure the inlining was
being effective at allowing the Rust compiler find opportunities to transform the code beyond what I could by hand.

Something that is discussed less is that I found opportunities for optimization when writing and reading to `standard I/O`. For example, it turns out that Rust default write function for `stdout` is 
line buffered. Meaning, as it is writing a buffer, it is also looking for newline (`\n`) characters as the signal to flush the buffer. This implicitly incurs a search pass penalty over our 
challenge (`2.2MB`) message that was not intended. As a result, I wrote a function to flush directly to the `stdout` file descriptor. In addition, I expanded the amount of bytes we ask per read of 
`stdin` to limit the number of times we hand execution back to the kernel. These things helped shave off at least a millisecond off the overall time.

We also switched the default global memory allocation from the OS (`glibc` in my case) to `MiMalloc` which is provided by the `mimalloc` crate. This crate was made by Microsoft and it is surprisingly 
ultra fast. It is the reason our total page faults dropped from ~3,500 to 700 or an 80% reduction. We have not discussed page faults, but they are another way in which we trigger kernel time and also 
stall the CPU until the data can be pulled. Minimizing it has effects on cache efficiency and overall execution time. I noticed during benchmarking that the crate might be using arenas under the hood. 
Further work on cleaning its usage with the smart pointers from the `rumtk-arena` crate might yield serious gains in the future. There is one trade off at the moment. Using this allocator meant that 
currently our processing time for a small message went from `800us` to `1.6ms` but this might be amortizable if processing multiple messages in a session on the same instance of the parser.
For our current workflows with one shot instances, it does decrease the performance substantially (going from 1100 to 550 messages a second per thread).

One idea that has not been explored is the parallelization strategy moving forward. I believe that with `tokio` we are armed with the ability to treat whatever number of threads that have been allocated 
as a single thread pool so we could simply emit discrete jobs from within the parser into the runtime. We could also emit discrete parsing events (`V2Message` creation) at the CLI level. Finally, we could 
make the current V2Message asynchronous to facilitate processing. Maybe, these ideas could lead to a faster overall processing time despite any penalties incurred by loading the job onto the 
runtime and executors.

Overall, I am very happy with current results and the parser is getting closer to the speed I expect of it on modern CPUs. As a disclaimer, my CPU is a consumer grade mid-level laptop/mobile CPU. I think 
that a nice Xeon or Epyc CPU could do wonders at our current level of optimization.
