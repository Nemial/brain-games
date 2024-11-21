using Nemial.BrainGames.Core;

namespace Nemial.BrainGames.Games;

public class Even : IGame
{
    private const string GameName = "Brain Even";
    private const string GameDescription = """Answer "yes" if the number is even, otherwise answer "no".""";

    private readonly GameEngine _engine = new();
    private readonly Random _random = new();

    public void Run()
    {
        var number = _random.Next(1, 256);
        var correctAnswer = IsEven(number) ? "yes" : "no";

        _engine.StartGame(GameName, GameDescription, number.ToString(), correctAnswer);
    }

    private static bool IsEven(int number)
    {
        return number % 2 == 0;
    }
}