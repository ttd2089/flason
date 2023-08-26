$tests = Get-ChildItem -Path "$PSScriptRoot/test-files" -Filter "*.json"
foreach($test in $tests)
{
    $expectationFile = $test.FullName -Replace ".json$", ".flason"
    
    $expected = Get-Content $expectationFile
    $actual = Get-Content $test | dotnet run --project "$PSScriptRoot/../../src/Ttd2089.Flason.Cli" -- -

    Compare-Object $expected $actual
}
