using Nemial.BrainGames.Core;

namespace Nemial.BrainGames.Games;

public class Progression : IGame
{
    public void Run()
    {
        Console.WriteLine("Добро пожаловать в Brain Progression!");
        Console.WriteLine("Определите спрятанное число число!");

        const int progressionLength = 10;
        const string hiddenValue = "...";


        var random = new Random();
        var step = random.Next(1, 5);
        var startNum = random.Next(1, 256);


        var progression = new List<string>();

        for (var i = 0; progression.Count < progressionLength; i += 1)
        {
            var progressionItem = startNum + i * step;
            progression.Add(progressionItem.ToString());
        }

        var hiddenKey = random.Next(0, progression.Count);
        var hiddenNum = progression[hiddenKey];
        progression[hiddenKey] = hiddenValue;

        Console.WriteLine(string.Join(", ", progression.ToArray()));
        var userInput = Console.ReadLine();
        var userAnswer = userInput?.ToLower().Trim();

        if (userAnswer == hiddenNum)
        {
            Console.WriteLine("Вы выиграли");
        }
        else
        {
            Console.WriteLine("Вы проиграли!");
            Console.WriteLine("Правильный ответ " + hiddenNum);
        }
    }
}