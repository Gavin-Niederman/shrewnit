def gen-unit-conv [ start, end ] {
    let lines = units -d 8 $'1($start)' $end | lines; 
    let ratios = $lines | each {|line| $line | parse --regex '\s*[\*/]\s*(?<ratio>[\d.e\-]*)' | get ratio } | flatten

    let mult = $ratios.0;
    let div = $ratios.1;

    echo $"($mult) per canonical\nper ($div) canonical"
}