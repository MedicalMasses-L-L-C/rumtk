# Project HIFLAMES: Building a Bridge to the Future (Part 2)

## Articles in Series
[Project HIFLAMES: Building a Bridge to the Future (Part 1)](./intro.md)
[Project HIFLAMES: Building a Bridge to the Future (Part 2)](./methods.md)
[Project HIFLAMES: Building a Bridge to the Future (Part 3)](./results1.md)
[Project HIFLAMES: Building a Bridge to the Future (Part 4)](./results2.md)

## Introduction:
Before going over the data on our HL7 parser, it is important we set the stage for our methodology and our architectural choices. The following article details how we structured our toolkit and how those decisions affect the speed of processing.

If you enjoy our work, please, visit the following pages. Perhaps, consider financially supporting our effort at our OpenCollective page.

* OpenCollective: https://opencollective.com/medicalmasses-llc/projects/rumtk-v2

* Website: https://www.medicalmasses.com/

* GiHub Repository: https://github.com/MedicalMasses-L-L-C/rumtk

Without further adieu...

## The Unix Philosophy:

Core to our work is the emphasis on structuring our tools into libraries, modules, and command-line interface programs (CLI). This structure follows the Unix Philosophy as developed by the giants at Bell Labs.

The idea of the Unix Philosophy is to break down a problem into their smallest components and build programs that excel at addressing them. Basically, you want to have programs to be masters of their problem space. Doing so, the hope is that fewer bugs arise at the component level because the feature permutation is clear and known ahead of time.

The downside of the Unix Philosophy is that it shifts a key portion of problem solving to the system level. As a result, individual programs are excellent at what they do, yet some kinds of bugs can still arise from complex interactions between these programs. Meaning, engineers must be very capable and knowledgeable to perform the occasional support of the tools. However, the system tends to be much more versatile and productive overall. It is the reason Cloud, Mac, and Linux based solutions are superior to Windows counterparts.

Additionally, the Unix Philosophy enables composition at the core of problem solving processes. For example, in a pure monolithic project, the tool might have a Word format parser but the technical engineer cannot directly use it to address a short term request from clients. Under the Unix Philosophy, a capable engineer can quickly deliver a solution without writing a single line of new code. Think about it. In our modern era of AI, the measure of productivity is Lines of Code (LoC) and yet an engineer empowered with the proper tooling achieves infinite productivity without a single line of new code.

## RUMTK:

Rust Universal Medical Toolkit (RUMTK) is a toolkit started and developed by Luis M. Santos, MD during his off hours since 2024. The goal of this framework is to rewrite the healthcare stack which started in the Windows era and thus took the wrong approach. This toolkit is inspired by the excellent work done by the OFFIS team on DCMTK. Luis noticed that we had proper tooling to work in the DICOM space but no analogue in the HL7 space. If we are to upgrade to FHIR, we need an equivalent set of tools.

RUMTK follows the Unix Philosophy to the teeth. So much so that even the output of individual tools are serialized (transformed to plain text) for easy inspection and usage by our engineers. Binary serialization will be introduced in the future to further optimize workflows when in automated mode.

RUMTK is broken down into a set of domain-specific libraries and CLI tools demonstrating the library usage. The CLI are strictly restricted to providing the recipe for applying the library tools with a very limited, tool-specific logic. That way, a bug can be quickly identified and problems are addressed with a systems architectures perspective.

### Libraries

* **rumtk-core** => Core modules and functionality library. For example, you can find here the basic functions, types, structs, and macros for managing the multi-threading runtime.

* **rumtk-hl7-v2** => Modules specifically built for implementing the HL7 V2 specification. The key type introduced is the V2Message type which performs the first parsing pass.

* **rumtk-web** => Modules meant for defining a fast and productive framework for building web applets in Rust. These applets contain facilities for pipelining and processing api calls as well as Server Side Rendering of websites. Our website was built using this framework. Applets created with the framework easily achieve low loading latency (as low as 10 ms on second load).

### CLI

rumtk-v2-interface => Tool for spinning an interface that can be chained with other tools to build a (quiet literal) pipeline. The idea is to organize customized solutions as if laying water pipes in a house.

rumtk-v2-parse => Tool for parsing a raw HL7 message into a fully searchable and (eventually) validated message ready for consumption by other tools such as a Machine Learning algorithm.

## The Message:

In these reports, we focus on **rumtk-hl7-v2** and **rumtk-v2-parse** as targets for optimization.

Our test dataset is an on-demand 2 MB V2 message generated with random data using an alphanumeric alphabet. The idea is to generate an atypically large message using readable ASCII characters to ensure we can test the actual parsing logic. The generated message is generated at the point of testing. The generated message contains 2048 OBX segments with 1024 KB of random data and a few bytes of segment header information.

The message is crafted to create a situation in that our parser has to identify the segment boundaries thousands of times. In addition, the total field and component boundary checks hit thousands of iterations as well. Each segment is much larger than an individual CPU cache-line although the whole segment should fit inside the total L1 cache for the CPU (see our [previous article](./intro.md) to learn more).

The segments are large enough that we expect some cache thrashing. Meaning, we should still see plenty of cache misses during iterations. The full message is too large to fit in L1 and maybe L2 caches forcing the CPU to have to pull segments from L3 down the cache hierarchy. Again, this creates a bit of a challenge due to the way CPUs have to evict and update cache-line while avoiding conflicts between cores. There is an excellent discussion of how caches affect performance in a multithreaded environment when say you need to lock a mutex. You can find it here.

Overall, the message represents a worse case scenario and a challenge to software architectures. Only engineers with the wealth and depth of knowledge such as it is found in MedicalMasses L.L.C. can navigate such a challenge. Getting this case right also means that a hospital system can send a large report to a screening algorithm and expect it to arrive fast and get processed fast.

**The synthetic message** uses this template:

### General Template
```
MSH|^~\&#|NIST EHR^2.16.840.1.113883.3.72.5.22^ISO|NIST EHR Facility^2.16.840.1.113883.3.72.5.23^ISO|NIST Test Lab APP^2.16.840.1.113883.3.72.5.20^ISO|NIST Lab Facility^2.16.840.1.113883.3.72.5.21^ISO|20130211184101-0500||OML^O21^OML_O21|NIST-LOI_9.0_1.1-GU_PRU|T|2.5.1|||AL|AL|||||LOI_Common_Component^LOI BaseProfile^2.16.840.1.113883.9.66^ISO~LOI_GU_Component^LOI GU Profile^2.16.840.1.113883.9.78^ISO~LAB_PRU_Component^LOI PRU Profile^2.16.840.1.113883.9.82^ISO
PID|1||PATID14567^^^NIST MPI&2.16.840.1.113883.3.72.5.30.2&ISO^MR||Hernandez^Maria^^^^^L||19880906|F||2054-5^Black or   African American^HL70005|3248 E  FlorenceAve^^Huntington Park^CA^90255^^H||^^PH^^^323^5825421|||||||||H^Hispanic or Latino^HL70189
ORC|NW|ORD231-1^NIST EHR^2.16.840.1.113883.3.72.5.24^ISO|||||||20130116090021-0800|||134569827^Feller^Hans^^^^^^NPI&2.16.840.1.113883.4.6&ISO^L^^^NPI
OBR|1|ORD231-1^NIST EHR^2.16.840.1.113883.3.72.5.24^ISO||34555-3^Creatinine 24H renal clearance panel^LN^^^^^^CreatinineClearance|||201301151130-0800|201301160912-0800||||||||134569827^Feller^Hans^^^^^^NPI&2.16.840.1.113883.4.6&ISO^L^^^NPI
DG1|1||I10^Essential (primary) hypertension^I10C^^^^^^Hypertension, NOS|||F|||||||||2
DG1|2||O10.93^Unspecified pre-existing hypertension complicating the puerperium^I10C^^^^^^Pregnancy with chronic hypertension|||W|||||||||1
{data}
SPM|1|S-2312987-1&NIST EHR&2.16.840.1.113883.3.72.5.24&ISO||276833005^24 hour urine sample (specimen)^SCT^UR24H^24hr Urine^99USL^^^24 hour urine|||||||||||||201301151130-0800^201301160912-0800
SPM|2|S-2312987-2&NIST EHR&2.16.840.1.113883.3.72.5.24&ISO||119297000^Blood Specimen^SCT|||||||||||||201301160912-0800ORC|NW|ORD231-2^NIST EHR^2.16.840.1.113883.3.72.5.24^ISO|||||||20130115102146-0800|||134569827^Feller^Hans^^^^^^NPI&2.16.840.1.113883.4.6&ISO^L^^^NPI
OBR|2|ORD231-2^NIST EHR^2.16.840.1.113883.3.72.5.24^ISO||21482-5^Protein [Mass/volume] in 24 hour Urine^LN^^^^^^24 hour Urine Protein|||201301151130-0800|201301160912-0800||||||||134569827^Feller^Hans^^^^^^NPI&2.16.840.1.113883.4.6&ISO^L^^^NPI
DG1|1||I10^Essential (primary) hypertension^I10C^^^^^^Hypertension, NOS|||F|||||||||2
```
### OBX Template
```
OBX|{line_number}|ED|4050097^Surg Path Final Report^^4050097^Surg Path Final Report||^PDF^^base64^{line}||||||F|||20120309132541
```
## The Benchmark Tools:

To benchmark and analyze the performance characteristics of our parser, we use a couple of common tools in Linux.

* **hyperfine**

* **flamegraph-rs**

* **perf record**

* **perf stat**

* **perf report**

**hyperfine** is a tool that is similar to the time command in Unix systems but it is specifically built for benchmarking other commands. It let's you define a set of warmup rounds which is done to ensure we minimize conflating variables from the system such as a random video frame filling the cache and unexpectedly evicting our message. This tool automatically runs the commands many times until the program settles at a true median time. It reports the min and max and standard deviation in time to help gauge how skew the data. For our experiment, we set a warmup period of 50 rounds and instruct it to call the program directly (skip opening a shell) to ensure measurements of the tool only accounts for events related to message parsing.
```
hyperfine -w 50 --export-markdown /dev/stdout --input /tmp/.tmp7Eelux/.tmp2rn4jX --output /dev/null --shell none ../target/release/rumtk-hl7-v2-parse
```

**flamegraph-rs** is a tool for generating an icicle or flamegraph visualization from the stack calls recorded by other tools such as perf. It facilitates analysis of where the program is spending its time. It's great for obtaining a global gross understanding of which functions are taking long during processing.
```
flamegraph -o /dev/stdout -i --deterministic --perfdata /tmp/.tmp7Eelux/.tmp7LXGiO
```

**perf** is a tool in Linux that can inspect CPU related events while your program is running and reports them using a binary format.

**perf record** is the command for running our tool and recording the stack frames and the number of cycles each function takes.
```
cat /tmp/.tmp7Eelux/.tmprhg3ku | perf record -F 999 --call-graph dwarf,64000 -g -o /tmp/.tmp7Eelux/.tmp7LXGiO ../target/release/rumtk-hl7-v2-parse
```

**perf stat** is the command for runnig your tool and reporting the basic CPU statistics such as cache misses.
```
cat /tmp/.tmp7Eelux/.tmpqRHcwy | perf stat -B -e cache-references,cache-misses,cycles,instructions,branches,faults,migrations -o /tmp/.tmp7Eelux/.tmpYRaNDH ../target/release/rumtk-hl7-v2-parse
```

**perf report** is the command used for generating a text based report about the CPU from the data generated from a prior perf record looking for cache misses.
```
cat /tmp/.tmp7Eelux/.tmpCiaz8L | perf record -s -e cache-misses,branch-misses -o /tmp/.tmp7Eelux/.tmpAuTiMn ../target/release/rumtk-hl7-v2-parse
```
```
perf report --stdio --header -I -v --percent-limit 1 -i /tmp/.tmp7Eelux/.tmpAuTiMn
```

## Conclusions:

Now that we have an understanding of how our software is architected, how we are challenging it, and how we are measuring it, we can focus on the first report analysis on the next article! Stay tune and help us out by going to the GitHub repository and liking it!