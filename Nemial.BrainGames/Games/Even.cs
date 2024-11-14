using Nemial.BrainGames.Core;

namespace Nemial.BrainGames.Games;

public class Even : IGame
{
    public void Run()
    {
        Console.WriteLine("Добро пожаловать в Brain Even!");
        Console.WriteLine("Напишите 'да', если число чётное, иначе 'нет'");
        var random = new Random();
        var number = random.Next(1, 256);
        var correctAnswer = IsEven(number) ? "да" : "нет";


        Console.WriteLine(number);

        var userAnswer = Console.ReadLine();

        if (userAnswer is null)
        {
            Environment.Exit(1);
        }

        var preparedAnswer = userAnswer.Trim().ToLower();

        if (preparedAnswer.Equals(correctAnswer))
        {
            Console.WriteLine("Вы выиграли!");
        }
        else
        {
            Console.WriteLine("Вы проиграли!");
            Console.WriteLine("Правильный ответ: " + correctAnswer);
        }
    }

    private static bool IsEven(int number)
    {
        return number % 2 == 0;
    }
}