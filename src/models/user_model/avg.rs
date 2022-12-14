pub fn model() -> Vec<Vec<[i32; 9]>> {
    vec![

        //  0 => General Stats
        vec![

                /*
                    Score Stats [i32; 9]

                    0 => List Length (anime watched)

                    1 => Overall Average Score,
                    2 => Overall Score deviation,
                    3 => Overall Scored %,


                    4, 5, 6, 7, 8 => ! Empty
                */

            //  0 => Score
            [651, 755, -34, 632, 0, 0, 0, 0, 0],

                /*
                    Statuses Stats [i32; 9]

                    0 => Status %,

                    1 => Status Average Score,
                    2 => Status Score deviation,
                    3 => Status Scored %,


                    4, 5, 6, 7, 8 => ! Empty
                */

            //  1 => Completed (status id: 1)
            [642, 758, -25, 915, 0, 0, 0, 0, 0],
            //  2 => Plan to Watch (status id: 2)
            [242, 756, -32, 64, 0, 0, 0, 0, 0],
            //  3 => Watching (status id: 3)
            [41, 783, -2, 449, 0, 0, 0, 0, 0],
            //  4 => On Hold (status id: 4)
            [30, 765, -65, 462, 0, 0, 0, 0, 0],
            //  5 => Dropped (status id: 5)
            [40, 711, -279, 648, 0, 0, 0, 0, 0],

            //  6 => Errors (when not empty some error occurred)
            [2, 2303, -105, 1, 0, 0, 1, 0, 0],
        ],

        //  1 => Airing Decades
        vec![
            //  0 => 1980s (0 - 1989)
            [23, 771, -37, 668, 546, 252, 22, 25, 19],
            //  1 => 1990s (1990 - 1999)
            [44, 781, -24, 627, 552, 263, 66, 48, 29],
            //  2 => 2000s (2000 - 2009)
            [204, 765, -32, 599, 607, 286, 30, 36, 33],
            //  3 => 2010s (2010 - 2015)
            [331, 755, -39, 660, 675, 229, 26, 27, 33],
            //  4 => 2020s (2016 - now)
            [392, 756, -42, 622, 582, 241, 63, 27, 48],
        ],

        //  2 => Ratings
        vec![
            //  0 => g (rating id: 1)
            [35, 744, -28, 661, 619, 245, 18, 19, 20],
            //  1 => pg (rating id: 2)
            [21, 745, -38, 738, 631, 170, 20, 25, 34],
            //  2 => pg 13 (rating id: 3)
            [595, 757, -36, 625, 625, 248, 44, 32, 43],
            //  3 => r (rating id: 4)
            [252, 774, -33, 646, 643, 239, 42, 30, 36],
            //  4 => r+ (rating id: 5)
            [77, 720, -54, 638, 620, 261, 27, 25, 45],
            //  5 => rx (rating id: 6)
            [13, 615, -97, 761, 313, 113, 12, 8, 27],
        ],

        //  3 => Series Length
        vec![
            //  0 => ~ 1 episode (1)
            [202, 754, -27, 687, 755, 214, 7, 5, 3],
            //  1 => ~ 6 episodes (2 - 8)
            [71, 729, -38, 663, 684, 227, 24, 18, 16],
            //  2 => ~ 12 episodes (9 - 18)
            [461, 746, -40, 628, 615, 250, 45, 30, 52],
            //  3 => ~ 24 episodes (19 - 32)
            [190, 786, -30, 610, 587, 265, 48, 44, 47],
            //  4 => ~ 48 episodes (over 33)
            [70, 795, -29, 604, 542, 251, 59, 70, 63],
        ],

        // 4 => Genres
        vec![
            //  0 => Action (genre id: 1)
            [152, 759, -39, 648, 645, 227, 45, 32, 44],
            //  1 => Adventure (genre id: 2)
            [74, 764, -37, 644, 636, 231, 45, 35, 43],
            //  2 => Avant_Garde (genre id: 5)
            [4, 765, -34, 684, 545, 256, 26, 26, 23],
            //  3 => Award_Winning (genre id: 46)
            [18, 825, -22, 696, 701, 226, 23, 18, 14],
            //  4 => Boys_Love (genre id: 28)
            [3, 704, -103, 731, 334, 186, 13, 12, 37],
            //  5 => Comedy (genre id: 4)
            [135, 749, -37, 628, 630, 244, 42, 32, 43],
            //  6 => Drama (genre id: 8)
            [102, 778, -28, 625, 631, 266, 34, 28, 33],
            //  7 => Fantasy (genre id: 10)
            [106, 753, -44, 652, 648, 222, 43, 32, 47],
            //  8 => Girls_Love (genre id: 26)
            [3, 694, -66, 667, 387, 253, 24, 24, 55],
            //  9 => Gourmet (genre id: 47)
            [3, 746, -47, 727, 506, 212, 29, 32, 46],
            //  10 => Horror (genre id: 14)
            [18, 724, -49, 657, 629, 246, 26, 28, 45],
            //  11 => Mystery (genre id: 7)
            [40, 771, -28, 600, 584, 289, 38, 34, 38],
            //  12 => Romance (genre id: 22)
            [87, 747, -48, 630, 625, 261, 32, 28, 43],
            //  13 => Sci_Fi (genre id: 24)
            [60, 757, -32, 598, 594, 288, 36, 31, 39],
            //  14 => Slice_of_Life (genre id: 36)
            [34, 766, -29, 584, 562, 305, 38, 32, 36],
            //  15 => Sports (genre id: 30)
            [13, 768, -33, 649, 562, 248, 46, 34, 46],
            //  16 => Supernatural (genre id: 37)
            [71, 769, -34, 634, 632, 250, 41, 31, 38],
            //  17 => Suspense (genre id: 41)
            [20, 798, -27, 672, 662, 229, 35, 30, 29],
            //  18 => Ecchi (genre id: 9)
            [31, 712, -72, 674, 633, 222, 26, 26, 60],
            //  19 => Erotica (genre id: 49)
            [0, 635, -126, 913, 280, 70, 15, 9, 44],
            //  20 => Hentai (genre id: 12)
            [6, 615, -97, 761, 314, 114, 12, 8, 27],
        ],

        //  5 => Themes
        vec![
            //  0 => Adult_Cast (genre id: 50)
            [42, 796, -25, 600, 573, 291, 45, 37, 30],
            //  1 => Anthropomorphic (genre id: 51)
            [8, 750, -36, 674, 511, 243, 37, 34, 53],
            //  2 => CGDCT (genre id: 52)
            [13, 756, -53, 625, 409, 282, 45, 34, 51],
            //  3 => Childcare (genre id: 53)
            [7, 817, -24, 642, 491, 257, 88, 31, 27],
            //  4 => Combat_Sports (genre id: 54)
            [3, 763, -39, 736, 332, 215, 28, 29, 32],
            //  5 => Crossdressing (genre id: 81)
            [1, 735, -48, 772, 392, 202, 19, 24, 35],
            //  6 => Delinquents (genre id: 55)
            [3, 802, -58, 702, 432, 256, 43, 45, 48],
            //  7 => Detective (genre id: 39)
            [10, 756, -38, 636, 466, 301, 34, 37, 40],
            //  8 => Educational (genre id: 56)
            [1, 732, -48, 772, 242, 165, 31, 30, 39],
            //  9 => Gag_Humor (genre id: 57)
            [18, 789, -38, 614, 513, 286, 51, 47, 45],
            //  10 => Gore (genre id: 58)
            [37, 768, -49, 709, 686, 188, 43, 24, 39],
            //  11 => Harem (genre id: 35)
            [40, 697, -94, 673, 565, 230, 27, 25, 74],
            //  12 => High_Stakes_Game (genre id: 59)
            [8, 749, -56, 742, 627, 199, 24, 26, 41],
            //  13 => Historical (genre id: 13)
            [40, 792, -30, 583, 552, 308, 43, 38, 36],
            //  14 => Idols_Female (genre id: 60)
            [4, 726, -58, 705, 301, 218, 24, 24, 44],
            //  15 => Idols_Male (genre id: 61)
            [1, 688, -63, 761, 136, 107, 14, 10, 27],
            //  16 => Isekai (genre id: 62)
            [33, 762, -62, 668, 609, 229, 45, 27, 49],
            //  17 => Iyashikei (genre id: 63)
            [16, 793, -30, 578, 443, 347, 38, 37, 34],
            //  18 => Love_Polygon (genre id: 64)
            [19, 761, -49, 667, 609, 250, 28, 31, 45],
            //  19 => Magical_Sex_Shift (genre id: 65)
            [0, 691, -59, 811, 238, 143, 21, 23, 44],
            //  20 => Mahou_Shoujo (genre id: 66)
            [9, 757, -48, 671, 456, 264, 28, 33, 38],
            //  21 => Martial_Arts (genre id: 17)
            [16, 739, -58, 679, 546, 221, 47, 50, 71],
            //  22 => Mecha (genre id: 18)
            [34, 758, -40, 624, 582, 282, 32, 31, 40],
            //  23 => Medical (genre id: 67)
            [1, 737, -47, 754, 279, 200, 31, 30, 45],
            //  24 => Military (genre id: 38)
            [38, 792, -32, 654, 634, 247, 36, 29, 34],
            //  25 => Music (genre id: 19)
            [24, 761, -33, 612, 567, 278, 38, 29, 37],
            //  26 => Mythology (genre id: 6)
            [36, 764, -45, 643, 617, 246, 37, 36, 42],
            //  27 => Organized_Crime (genre id: 68)
            [8, 790, -40, 678, 550, 258, 35, 36, 34],
            //  28 => Otaku_Culture (genre id: 69)
            [13, 764, -38, 664, 559, 250, 43, 34, 39],
            //  29 => Parody (genre id: 20)
            [22, 772, -42, 660, 619, 235, 39, 35, 36],
            //  30 => Performing_Arts (genre id: 70)
            [4, 774, -32, 655, 355, 301, 28, 27, 32],
            //  31 => Pets (genre id: 71)
            [0, 732, -44, 804, 253, 143, 13, 14, 19],
            //  32 => Psychological (genre id: 40)
            [65, 775, -29, 649, 641, 247, 35, 30, 34],
            //  33 => Racing (genre id: 3)
            [1, 757, -45, 786, 290, 171, 14, 16, 20],
            //  34 => Reincarnation (genre id: 72)
            [11, 779, -67, 724, 545, 197, 49, 23, 42],
            //  35 => Reverse_Harem (genre id: 73)
            [4, 714, -69, 722, 357, 212, 25, 26, 60],
            //  36 => Romantic_Subtext (genre id: 74)
            [12, 794, -45, 727, 649, 189, 39, 23, 38],
            //  37 => Samurai (genre id: 21)
            [10, 793, -44, 627, 441, 302, 46, 52, 44],
            //  38 => School (genre id: 23)
            [177, 755, -42, 652, 648, 230, 37, 30, 44],
            //  39 => Showbiz (genre id: 75)
            [2, 781, -20, 752, 432, 222, 20, 19, 23],
            //  40 => Space (genre id: 29)
            [12, 776, -37, 595, 419, 332, 44, 40, 34],
            //  41 => Strategy_Game (genre id: 11)
            [9, 759, -39, 697, 591, 221, 32, 35, 46],
            //  42 => Super_Power (genre id: 31)
            [51, 760, -45, 669, 654, 220, 37, 31, 41],
            //  43 => Survival (genre id: 76)
            [20, 775, -51, 754, 711, 160, 30, 22, 41],
            //  44 => Team_Sports (genre id: 77)
            [12, 782, -42, 708, 502, 212, 51, 34, 46],
            //  45 => Time_Travel (genre id: 78)
            [16, 800, -41, 679, 631, 223, 43, 32, 37],
            //  46 => Vampire (genre id: 32)
            [17, 756, -42, 667, 604, 240, 37, 35, 43],
            //  47 => Video_Game (genre id: 79)
            [11, 731, -81, 736, 591, 191, 26, 27, 62],
            //  48 => Visual_Arts (genre id: 80)
            [6, 771, -34, 657, 499, 279, 28, 29, 34],
            //  49 => Workplace (genre id: 48)
            [10, 763, -37, 619, 484, 295, 43, 39, 44],
        ],

        //  6 => Demographics
        vec![
            //  0 => Josei (genre id: 43)
            [23, 763, -38, 625, 381, 317, 31, 29, 37],
            //  1 => Kids (genre id: 15)
            [29, 704, -40, 784, 488, 113, 20, 26, 37],
            //  2 => Seinen (genre id: 42)
            [242, 763, -33, 627, 614, 262, 38, 31, 40],
            //  3 => Shoujo (genre id: 25)
            [98, 765, -50, 598, 516, 306, 37, 36, 46],
            //  4 => Shounen (genre id: 27)
            [600, 774, -39, 665, 655, 205, 52, 36, 43],
        ],
    ]
}
