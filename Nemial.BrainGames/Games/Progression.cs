using Nemial.BrainGames.Core;

namespace Nemial.BrainGames.Games;

public class Progression : IGame
{
    private const string GameName = "Brain Progression";
    private const string GameDescription = "What number is missing in the progression?";

    private const int ProgressionLength = 10;
    private const string HiddenValue = "...";

    private readonly GameEngine _engine = new();
    private readonly Random _random = new();

    public void Run()
    {
        var progression = GenProgression();
        var hiddenKey = _random.Next(0, progression.Count);
        var hiddenNum = progression[hiddenKey];
        progression[hiddenKey] = HiddenValue;

        var question = string.Join(", ", progression.ToArray());

        _engine.StartGame(GameName, GameDescription, question, hiddenNum);
    }

    private List<string> GenProgression()
    {
        var step = _random.Next(1, 5);
        var startNum = _random.Next(1, 256);
        var progression = new List<string>();

        for (var i = 0; progression.Count < ProgressionLength; i += 1)
        {
            var progressionItem = startNum + i * step;
            progression.Add(progressionItem.ToString());
        }

        return progression;
    }
}