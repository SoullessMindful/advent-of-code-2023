import System.IO
import Data.Maybe (catMaybes)
import System.Environment (getArgs)
import Data.Bifunctor (Bifunctor(second, bimap, first))


(<|) = id
(|>) arg func = func arg


maxBalls = RGB 12 13 14


main = do
  args <- getArgs
  file <- openFile (head args) ReadMode
  text <- hGetContents file
  print <| processText text
  return 0


processText :: String -> Int
processText text = lines text
  |> map processLine
--  |> catMaybes
  |> sum


processLine :: String -> Int
processLine line = splitOnceOn ':' line
--  |> bimap extractId processTakes
  |> snd
  |> processTakes
  |> (\lineBalls -> red lineBalls * green lineBalls * blue lineBalls)


extractId :: String -> Int
extractId s = splitOnceOn ' ' s
  |> snd
  |> read :: Int


processTakes :: String -> Balls
processTakes s = splitOn ';' s
  |> map parseBalls
  |> maximumBalls


data Balls = RGB Int Int Int

instance Num Balls where
  RGB a b c + RGB d e f = RGB (a+d) (b+e) (c+f)
  RGB a b c - RGB d e f = RGB (a-d) (b-e) (c-f)
  RGB a b c * RGB d e f = RGB (a*d) (b*e) (c*f)
  abs (RGB a b c)  = RGB (abs a) (abs b) (abs c)
  signum (RGB a b c)  = RGB (signum a) (signum b) (signum c)
  fromInteger i = RGB (fromInteger i) 0 0


instance Ord Balls where
  RGB a b c <= RGB d e f = (a<=d) && (b<=e) && (c<=f)


instance Eq Balls where
  RGB a b c == RGB d e f = (a==d) && (b==e) && (c==f)


instance Show Balls where
  show (RGB r g b) = "r: " ++ show r ++ ", g: " ++ show g ++ ", b: " ++ show b


red (RGB r g b) = r
green (RGB r g b) = g
blue (RGB r g b) = b


maximumBalls :: [Balls] -> Balls
maximumBalls [] = RGB 0 0 0
maximumBalls balls = RGB
  (balls |> map red |> maximum) 
  (balls |> map green |> maximum) 
  (balls |> map blue |> maximum) 



parseBalls :: String -> Balls
parseBalls s = splitOn ',' s
  |> map parseBall
  |> sum


parseBall :: String -> Balls
parseBall s = s
  |> tail
  |> splitOnceOn ' '
  |> first read
  |> (\(num,col) -> case col of
    "red" -> RGB num 0 0
    "green" -> RGB 0 num 0
    "blue" -> RGB 0 0 num
    _ -> RGB 0 0 0
  )


splitOnceOn :: Char -> String -> (String, String)
splitOnceOn c s = break (==c) s
  |> second safeTail


splitOn :: Char -> String -> [String]
splitOn c s = if t == ""
  then [h]
  else h : splitOn c t
  where (h,t) = splitOnceOn c s


safeTail :: [a] -> [a]
safeTail [] = []
safeTail l = tail l
