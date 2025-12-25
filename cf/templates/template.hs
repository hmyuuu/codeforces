import Control.Monad (replicateM_)

solve :: IO ()
solve = do
    return ()

main :: IO ()
main = do
    t <- readLn :: IO Int
    replicateM_ t solve
