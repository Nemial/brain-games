using Nemial.BrainGames.Core;

namespace Nemial.BrainGames.Games;

public class Prime : IGame
{
    private const string GameName = "Brain Prime";
    private const string GameDescription = """Answer "yes" if given number is prime. Otherwise answer "no".""";

    private readonly Random _random = new();


    public void Run()
    {
        var num = _random.Next(512);
        var answer = IsPrime(num) ? "да" : "нет";

        GameEngine.StartGame(GameName, GameDescription, num.ToString(), answer);
    }

    private static bool IsPrime(int number)
    {
        if (number < 2)
        {
            return false;
        }

        for (var i = 2; i < number; i += 1)
        {
            if (number % i == 0)
            {
                return false;
            }
        }

        return true;
    }
}