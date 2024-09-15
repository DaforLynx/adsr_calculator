attack_table = [255, 254, 253, 252, 251, 250, 249, 248,
                247, 246, 245, 244, 243, 242, 241, 240,
                239, 238, 237, 236, 235, 234, 233, 232,
                231, 230, 229, 228, 227, 226, 225, 224,
                223, 222, 221, 220, 219, 218, 217, 216,
                215, 214, 213, 212, 211, 210, 209, 208,
                207, 206, 205, 204, 203, 202, 201, 200,
                199, 198, 197, 196, 195, 194, 193, 192,
                191, 190, 189, 188, 187, 186, 185, 184,
                183, 182, 181, 180, 179, 178, 177, 176,
                175, 174, 173, 172, 171, 170, 169, 168,
                167, 166, 165, 164, 163, 162, 161, 160,
                159, 158, 157, 156, 155, 154, 153, 152,
                151, 150, 149, 148, 147, 143, 137, 132,
                127, 123, 116, 109, 100, 92,   84,  73,
                63,   51,  38,  26,  14,  5,    1,   0]

decay_table =   [  1,    3,    5,    7,    9,   11,    13,    15,
                  17,   19,   21,   23,   25,   27,    29,    31,
                  33,   35,   37,   39,   41,   43,    45,    47,
                  49,   51,   53,   55,   57,   59,    61,    63,
                  65,   67,   69,   71,   73,   75,    77,    79,
                  81,   83,   85,   87,   89,   91,    93,    95,
                  97,   99,  101,  102,  104,  105,   107,   108,
                 110,  111,  113,  115,  116,  118,   120,   122,
                 124,  126,  128,  130,  132,  135,   137,   140,
                 142,  145,  148,  151,  154,  157,   160,   163,
                 167,  171,  175,  179,  183,  187,   192,   197,
                 202,  208,  213,  219,  226,  233,   240,   248,
                 256,  265,  274,  284,  295,  307,   320,   334,
                 349,  366,  384,  404,  427,  452,   480,   512,
                 549,  591,  640,  698,  768,  853,   960,  1097,
                1280, 1536, 1920, 2560, 3840, 7680, 15360, 65535]

sustain_table = [   -92544, -92416, -92288, -83328, -76928, -71936, -67840, -64384,
                    -61440, -58880, -56576, -54400, -52480, -50688, -49024, -47488,
                    -46080, -44672, -43392, -42240, -41088, -40064, -39040, -38016,
                    -36992, -36096, -35328, -34432, -33664, -32896, -32128, -31360,
                    -30592, -29952, -29312, -28672, -28032, -27392, -26880, -26240,
                    -25728, -25088, -24576, -24064, -23552, -23040, -22528, -22144,
                    -21632, -21120, -20736, -20224, -19840, -19456, -19072, -18560,
                    -18176, -17792, -17408, -17024, -16640, -16256, -16000, -15616,
                    -15232, -14848, -14592, -14208, -13952, -13568, -13184, -12928,
                    -12672, -12288, -12032, -11648, -11392, -11136, -10880, -10496,
                    -10240,  -9984,  -9728,  -9472,  -9216,  -8960,  -8704,  -8448,
                     -8192,  -7936,  -7680,  -7424,  -7168,  -6912,  -6656,  -6400,
                     -6272,  -6016,  -5760,  -5504,  -5376,  -5120,  -4864,  -4608,
                     -4480,  -4224,  -3968,  -3840,  -3584,  -3456,  -3200,  -2944,
                     -2816,  -2560,  -2432,  -2176,  -2048,  -1792,  -1664,  -1408,
                     -1280,  -1024,   -896,   -768,   -512,   -384,   -128,      0]

FPS = 192
sustain_value: int = 0
velocity_value: int = 127

for attack in attack_table:
    steps: int = 0
    velocity = velocity_value
    while velocity > int(0):
        steps += 1
        velocity = attack * velocity // int(256)
    print(steps/FPS)

for decay in decay_table:
    steps: int = 0
    sustain = sustain_table[sustain_value]
    velocity = 0
    while velocity > sustain:
        steps += 1
        velocity -= decay
    print(steps/FPS)

for release in decay_table:
    steps: int = 0
    zero: int = -92544
    velocity = 127
    while velocity > -92544:
        steps += 1
        velocity -= release
    print(steps/FPS)
