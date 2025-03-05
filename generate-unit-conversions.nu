def gen-unit-conv [ start, end, digits = 15 ] {
    let lines = units -o $"%.($digits)g" $'1($start)' $end | lines; 
    let ratios = $lines | each {|line| $line | parse --regex '\s*[\*/]\s*(?<ratio>[\d.e\-]*)' | get ratio } | flatten

    let mult = $ratios.0;
    let div = $ratios.1;

    echo $"($mult) per canonical\nper ($div) canonical"
}