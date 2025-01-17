nums :: [Int]
nums = [x | x <- [0 .. 10], x /= 2]

--
--
-- comp = [ mapping | binding, condition* ]
-- mapping = expr
-- binding = pattern '<-' iterable
-- pattern = expr
-- condition = (true & condition).and_then();
--
--
