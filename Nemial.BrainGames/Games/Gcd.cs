using Nemial.BrainGames.Core;

namespace Nemial.BrainGames.Games;

public class Gcd : IGame
{
    private const string GameName = "Brain GCD";
    private const string GameDescription = "Find the greatest common divisor of given numbers.";

    private readonly Random _random = new();

    public void Run()
    {
        var random = new Random();
        var firstNum = random.Next(1, 512);
        var secondNum = random.Next(1, firstNum);
        var gcd = FindGcd(firstNum, secondNum);
        var question = $"{firstNum} {secondNum}";

        GameEngine.StartGame(GameName, GameDescription, question, gcd.ToString());
    }

    private static int FindGcd(int firstNum, int secondNum)
    {
        return firstNum % secondNum != 0 ? FindGcd(secondNum, firstNum % secondNum) : secondNum;
    }
}