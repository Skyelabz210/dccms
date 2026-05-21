# SLUB IIIF single-page probe — v0.9.5 N15
$ErrorActionPreference = 'Stop'
$url = 'https://digital.slub-dresden.de/data/kitodo/codedrm_280742827/codedrm_280742827_tif/jpegs/00000001.tif.original.jpg'
$outDir = Join-Path $HOME 'Agents\imports\slub_dresden'
if (-not (Test-Path $outDir)) { New-Item -ItemType Directory -Path $outDir | Out-Null }
$out = Join-Path $outDir 'page_00000001.jpg'
try {
    $r = Invoke-WebRequest -Uri $url -OutFile $out -PassThru -UserAgent 'dccms-research/0.9.5 (Skyelabz210)'
    $size = (Get-Item $out).Length
    Write-Output ("OK status={0} bytes={1}" -f $r.StatusCode, $size)
} catch {
    Write-Output ("FAIL: " + $_.Exception.Message)
}
