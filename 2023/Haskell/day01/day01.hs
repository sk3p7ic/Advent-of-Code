import System.IO

-- Thanks to https://www.youtube.com/watch?v=hTCMYOgFY5o for the code help in
-- attempting to learn even just a little Haskell
import Data.Char (isDigit, digitToInt)

toNumber :: [Char] -> Int
toNumber = ((+) <$> ((*10) . head) <*> last) . map digitToInt

part1 :: [String] -> Int
part1 = sum . map (toNumber . filter isDigit)

main :: IO ()
main = do
    rawContents <- readFile "../../Rust/problems/inputs/day01.in.txt"
    print . part1 $ lines rawContents
