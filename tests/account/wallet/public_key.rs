use lunesrs::wallet::assembly::to_public_key;

use wasm_bindgen_test::wasm_bindgen_test;

#[test]
#[wasm_bindgen_test]
fn multple_public_key() {
    let private_key = [
        [
            [
                160, 66, 17, 225, 21, 144, 128, 203, 241, 21, 205, 209, 16, 138, 219, 155, 50, 48,
                24, 209, 227, 79, 35, 104, 252, 102, 213, 74, 63, 165, 20, 96,
            ],
            [
                28, 105, 36, 199, 36, 111, 120, 95, 152, 208, 215, 39, 161, 71, 78, 237, 200, 160,
                71, 209, 177, 102, 140, 170, 56, 206, 9, 214, 227, 38, 117, 117,
            ],
        ],
        [
            [
                152, 195, 158, 43, 235, 175, 81, 113, 71, 142, 134, 117, 226, 247, 140, 189, 9, 86,
                193, 54, 59, 40, 100, 59, 213, 171, 8, 113, 151, 244, 43, 116,
            ],
            [
                138, 251, 177, 135, 204, 17, 215, 139, 107, 110, 163, 159, 69, 66, 230, 125, 46,
                90, 155, 251, 112, 76, 80, 226, 246, 159, 0, 247, 24, 204, 238, 127,
            ],
        ],
        [
            [
                48, 135, 161, 15, 52, 78, 234, 177, 234, 101, 67, 192, 68, 174, 104, 124, 28, 156,
                23, 33, 81, 118, 210, 255, 127, 127, 59, 24, 148, 215, 25, 77,
            ],
            [
                83, 143, 55, 207, 188, 113, 76, 98, 188, 187, 21, 6, 121, 237, 114, 87, 56, 119,
                247, 123, 107, 235, 127, 93, 111, 125, 177, 254, 234, 7, 182, 102,
            ],
        ],
        [
            [
                64, 188, 249, 142, 153, 123, 119, 187, 134, 139, 142, 224, 144, 233, 96, 219, 118,
                79, 3, 179, 172, 145, 191, 189, 235, 205, 232, 119, 176, 55, 76, 69,
            ],
            [
                24, 17, 29, 210, 50, 221, 206, 124, 241, 169, 109, 116, 202, 228, 241, 10, 66, 235,
                31, 179, 74, 61, 220, 114, 110, 17, 25, 9, 161, 78, 24, 115,
            ],
        ],
        [
            [
                176, 132, 66, 150, 25, 7, 98, 166, 0, 121, 84, 17, 161, 132, 204, 58, 19, 4, 158,
                161, 26, 205, 63, 198, 230, 171, 218, 199, 199, 217, 26, 102,
            ],
            [
                199, 51, 26, 241, 231, 42, 46, 169, 1, 155, 227, 85, 160, 76, 123, 191, 181, 159,
                48, 66, 209, 156, 162, 79, 235, 66, 199, 211, 35, 21, 161, 56,
            ],
        ],
    ];

    for x in private_key {
        assert_eq!(to_public_key(x[0].to_vec()), x[1]);
    }
}

#[test]
#[wasm_bindgen_test]
fn single_public_key() {
    let private_key: Vec<u8> = vec![
        160, 66, 17, 225, 21, 144, 128, 203, 241, 21, 205, 209, 16, 138, 219, 155, 50, 48, 24, 209,
        227, 79, 35, 104, 252, 102, 213, 74, 63, 165, 20, 96,
    ];

    assert_eq!(
        to_public_key(private_key),
        [
            28, 105, 36, 199, 36, 111, 120, 95, 152, 208, 215, 39, 161, 71, 78, 237, 200, 160, 71,
            209, 177, 102, 140, 170, 56, 206, 9, 214, 227, 38, 117, 117,
        ]
    );
}
