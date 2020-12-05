fn main() {
    const LAYER_SIZE: usize = 25 * 6;

    let mut layers: Vec<Vec<u32>> = Vec::new();

    //let imagedata = "123456789012";

    let imagedata = "222222222222220222122222220022222222222202222222222222220202222222222201212222120222222220222102222212222222222012222222002222221222222202222220022220222222222222222222122222220222222222022202222222222222220222222222222222202222221222222221222202222212222222222102222222202222220222222222222221222220222222222222221222122222221022222222222212222222222222221212222222222212222202220222220220222112222202222222222122222222122222220222222202222222222222222222222222221222122222221022222222122222222222222222221102222222222201202212021222220220222212222202222222222002222222202222220222222202222221222222222222222222221222222222222122222222022202222222222222220202222222222200202202121222222221222202222222222222222022222222012202220222222202222222122222222222222222222222222222222022222222122212222222222222220112222222222221222202221222221221222122222222222222222112222222212212222222222102222222222221222222222222220222022222220022222222022222222222222222221102222222222201222212020222222222222002222202222222222022222022002012222222222012222220222222222222222222221222222222221122222222220212022222222222220012222222222200202212220222222220222212212202222222222002222122002212222222222222222220222221222222222222220222222222220222222222020212022222222222220002222222222211222222022222221221222202222222222222222002222022212102221222222012222220122222222222222222220222122222221122222222120212022222222222022012222222222200202222221222220222202102202202222222222202222022222022220222222212222220222222222222222222220222022222222122222222220212222222222222222002222222222221202212220222221221202012222222222222222022222122202002221222220002222221222220222222222222221222222222221222222222020212222222222222222102222222222200202212220222222220202112202212222220222012222222212112221222222212222220022220222222222222220222022222221122222222120212122222222222020212222222222200002202221222220222212222202222222220222212222222001012221222220202222120122221222222222222221222202222222122222222220222022222222222022102222222222221222222020222221222212012202212222221222112222122011012221222221222222211022220222222222222220222122222122122222222221222122222222222021112222222222220002212221222221220222122202202222221222002222122020012221222220112222111222222212222222222220222002222220122222222221222222222222222021222222222222201012212020222220220222002222202222222222202222122121212220222220102222002122222202222222222222222022222222122222222020212222222222222222202222222222211002202121222221221222102212202222221222012222022202122220222221222220001022221202222222222221222222222220122222222121222222222222222021122222222222202012211120222221220202102222212222221222002222122110012222222222212222120222221222222222222220222112222222022222222222212122222222222120202222222222210012222220222221222202112212202222221222222222022022122220222220222222000022221212222222222221222012222020222222222120212022222222222120202222222222202222202220222221221202012002212222222222122222222012102220222220012222121122222202222222222222222002222022222222222120202022222222222121212222222222202122222220222222221202002102222222220222212222122212022221222220012221220022222212222221222221222012222122222220222121212022222222020120222222222222222202200120220220222212112102212222222222212222122120002221222222022221101122221222222220222222222122222222122220222221212022222222020220102222222222222112221122220222221202212002202222222222222222022022212222222221012222001122220212222220222220222002222121222222222120222022222222020120222222222222222002221120221221221222202222202222222222012222022201122221222220112221002222221222222220222221222222222122122221222120222022222222022020102222222222222222211122222222221212012202212222222222002222222002222220222221112222011122220222222222222220222202222122022220222022222220222222021222202222222222210222210021220221222222022212222222221222212222122012012222222220122221220222121202222220212221222122222120022221222122222122222222121022212222222222220012222020221220220202212122222222222222222222022210202221222220212220220222220222222220202221222222222021222221222120212220222222121120202222222222220122212121221221221212102122202222221222212222122101102221222222212222200222021212222221222222222022222022022221222022222220222222121220122222222222202222221122220222220222112102212212222222022222222200112222222222102221202222220222222221202222222212222210122222222220222220222222121221212222222222220110222022201222222222202012212222220222112222222111222220222220112221101222021212222221202221222112222022122222222121222121222222021120122222222222200110102121210221222222112222202212220222202222022101102221222220102221110122101212222121212221222112222011020220222022202022222222120020222222222222212212011020210221221222202102222222220222222222222111122220222221202221121122022202222020222221222202222211222220222021202222222222220220122222202222210222122120221222221202012012222222222222012222122100212221222221002221212022000212222221202221222222222100020222222020222120222222122121212222212222212122220120212220222212012012222022221222012222022001122220222221102221012222220202222220202222222122222012220220222022222121222222020022102222222222210220120020220221222212212002212102220222202222122000012222222221122222200022221202222122212222222212222201022221222021212121222212120222112222222222210001002122220220222212222012222112121222222222122020012220222222002222101022210212222022212220222122222000021222222122212220222222122222202222202222202112221020210222202202012212212122121222012222222020022221222222212220010222122212222221222221222102222220222222221012202221122212121021202222222222222210221122211220200212212102202122221222022222022211202221222222002220011022001222202120212220222212222110112202220111222222022222120022102222202222212102012221220221210222012122212012221222112222222120202222222222022221101022011202202220222221222202222210200201221012202120122212022220112222212222201120121121220222222212212022222002220222012222022200122220222221122221112022120222222221222222222212222212001201222112222121222202120122012222222222221011200120210221210212012022222002220222012222122220122221222221122221002022202212202122222220222202222121221210121010222222122212220020222222202222222000121021210220222212112022222202020222012222122000002220222222122220020122101212201221212222222212222010220220022202202120122222122221122222212222210022211222210222202222212002222002220222222222022121022221222220012222102222211212202221222220222222222011220221221012212222222212020021212222202222221201020121212221210212202112222102020222002222122202122221222221112221200122121202210122202222222202222211110221021022212122222212020122022222212222211022212220212220211202222222222212022222002222222001022220222222212222020222220212221021212222222122222101001212122102202121222222020222222222212222220101000222222222212212002022222002022222002222022220002120222221122221112022002222211122202222222012222122220211222222212122122222022222112222202222212122011021212222211212022202202202122222022222122120012220222221102220102022001202202222202220222222222210112002022120222221022202221120202222212222220212221222211220202212012222202002020222102222022211012020222221012220201022010202222022222220222112222022120100020120212020022212022221022222212222221110112020210221212222112102202022222222222222022210012021222220202222022122220202220221222222222202222120020002222212212022222202121022222222202222221022120021200220220212012122212122020222002222022200212120222222012222012022021222200022202221222012222101001020020221202122122222122220012222002221211221111021222221211202112122222122220222202222222011212120222222222221100222210202200020222222222012222100112002021121222122022222022122022222102220220221102121202221202202112112202112221222122222022022122221222220222220110222021222201221212221222022222102110000020201222020022202020022222222112222222221221122221221200212112122222002120221012222022212102222222220212222021022022212202022202222222102222020120011120022222122222202121221012222202220201122102120201220221212202002202122220221202222022110022121222220112221021022121212211222202222222102222200220200221012210221222222120102002222102222211221201020211221210212022222202202221220102222222020212022222220102221121222020212201220202220222112222211102010021112202222122222122201222222122220212122001221201221212212012222202112221222122222122201022222222222202220110122112222212121212221222222222220121120120010200120022202122122212222112220222120002222222220220222112212212102220222112222022201122022222221022222212122022202212221222221222112222110111112220220210120222202020100022222022022221202011020220220220222212102222112022220122222222220202220222222112221022122221202210122212220222102220021000202022120221120022222221221022222122120201120111022210222220222202212212112020221022222022001122022222220022220210122111202212220202220222202222110222201120111222121122222020210102222022120210100101020212221221222112102212112220222012222122202122220222222022220122220220222212122202222222112221111102210021002202020122222221100112222212122210200022120202222220222002022222222222222112222122000112020222222112221102021002202200020222220222022220012001111120001220020222202121011212222122202212120020222221220202212202002202102220222122222222001002222222220212221120220111012220221222222222222222200010011222211211121222202022010002222222022220012100120211222211212222012202022220220012222222202102120222221110202212120112202202222222222022212222212001020121020211221222222020222212222012110200100102122200222201212122202022012022221122222122112102011222220202220221022122222210120202222222112222000202021220100220221222202122110202222102212200210221021201222202222212202022022220220122212022100202111222222022200101020110201220020222221122002221212212112021220220221222212222220002222102200211000120121220222212202212022212022221221202202022111002012222221122201022222022002002020222221122022221220102222222201201122122222122020012222212100212200202221212221210212122112022122220221122212022100112020222221000210211121212210020220202221222012220012200211120102212122222202022122202222122120202010211020200220222222102222112222022220222202022201200110222221200212222221110200012221222221222222221111102021120000200221222202020001222222022010222212121122212220202202112222022122120220222202022021220021222220210202001220012012220222212221022112221200122222022011210222122212020000012222002200220202210121202221202202102202102112220222102212022122000222222220112202200121222112111022202220122002221020000200121011220220222212222010002222202120222212112022202221222222212122122112220220122202022102200121222220001210122220010202200221002221122112221021112212020111201122122212120102022222212010212102201222220222112222022202112002120221112202022222112210222222102211220222111101121222012220222002221211111022220010212020222222220220102222012222200010200121200222110212022202002202121222010202122202112120222220001211002120222100010221102021122222222211101021221001200121022212020101002222012122201212011222111221002202112012002112022221221222222000212102222220100201120122210021012021122022022012220102221221121211200120122212020001202222122000211021201121120220112222122112202212222220122222222121201110222222122210102121102111200220102221022012221002221110022022201011122222022102012222002211221221202220221221210212102102222202222222011222222001201012222220222212010122112202010221212221122212221122200002120000020200022212021012002220002110200112102121121221201222002212102202120222121202122012211212222220210200022021201102000022102121022002220022122122121222221100122212022002022122002102200222222220212221001222222102102212120202012212122022221112222221210211200121211211222122112221022102221012122122220010102011222222121120112220122102222121121020221221010222122202122112121211202222122100221010222220102222220122010200011120122220222102221221221121022102000001122222221100222121102122211012212221120220212202212112202002020211022202022221211000222221111220222122212102110121122022122202222120010121020121210101222212120101012120212211200120120122102221111222122112112122021221120212222201101011222221112222020120011022011221212120022102222110022220021012212210022212022202002222112202202100200120200222201212002022022112122221102202122221121111222220111210101121122021021120112122022112220111202102222021020020122222120110122222222111200022002120020221212212012022022022121210212212122201112202222222202111011021202222101121112222122202220020221212221201020211022202222211202120212121201120111120102222200202222012012022121202101212222000200111222220211111102122100212222022122120222012222200100220020220022212222222022121202021202122211022101022200220020222022112012022120202101212022012100211222221111021211120111021011120102021022102222112111100022012112111122222020121012022012200201002000022101221201212012012112102120200022202022021010212222221211202121121021012212120112220021102220111102011022221220110222222222102212221112021222120011222211222020222222002122122021200102202222102011112222220202110202121021201000222102120020112220120110202222210020101022202221012012021102112211100100020101221210202222112102112120220002212122211022120222220122200210222210111212222122222120212021102112122220212011220222212121112212220102111201201002122222220220202222212102202120211201202222202010211222222011011120100212110211220112020020102021120202222222100201120022202222020112021002012221120201221212221110222002022202222020202122212122210110011222222101201210111211101022222002120221122020210112112022220101102222212120212012020102201212010210221111222110222212012002012220221011212022112110012222222212021212100002102202122012020121112021002201222021221220112122212021120102120002212210121200222221222212202222022022100022201010202222112001210222212220201201001021110112122102120001122222221010102121221002120222212022212112221202200211120202022202221011212022012122101021220100202122212111011222200122211101012121111111122112221200122022211101021221020201222022202220221112122022121212200001221222221000222212002212212122010212222122122101212222220121020020111100000110122102120122202220201221011222212020221220212022221102221202212202121101122010220001212012212112201021211000022222220202100222211112020021221121122202121022222022222220102022002122120211011220212020021022222102022222210012222101021222212012012122111121110120122122202001101222220020112021102002110210221212220002022221122001011220212000021222212221102222122012110201020121020100122222202022222202211121110212102022012122112222100101202010201121110010220102222202022220111201122021000221102222212220111202220122122202021202120021221021212212102102102122222212222222212201122222202021210110222112011121221012020001222020022201201101210010202020202000102222120022010220201121220122221122212112012112211120211200202022202120021222021222012112012110120010012210212121121201011020012122110012202102011101220220212200121020000101100100111012010120120000122100110011200110100002001011101102110002120111";

    let image_as_vec: Vec<u32> = imagedata
        .chars()
        .map(|x| x.to_digit(10).unwrap_or(0))
        .collect();
    let iter = image_as_vec.chunks(LAYER_SIZE);
    for layerdata in iter {
        let v = layerdata.to_vec();
        println!("{:?}", v);
        layers.push(v);
    }
    println!("{:?}", layers.len());

    let mut output = vec![9; LAYER_SIZE];
    for p in 0..LAYER_SIZE {
        for l in 0..layers.len() {
            if layers[l][p] == 2 {
                continue;
            } else {
                output[p] = layers[l][p];
                break;
            }
        }
    }

    for y in 0..6 {
        for x in 0..25 {
            if output[y * 25 + x] == 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
