using Nemial.BrainGames.Core;

namespace Nemial.BrainGames.Games;

public class Calc : IGame
{
    private const string GameName = "Brain Calc";
    private const string GameDescription = "Brain Calc";

    private readonly Random _random = new();
    private readonly string[] _operations = ["+", "-", "*"];

    public void Run()
    {
        var firstNum = _random.Next(1, 512);
        var secondNum = _random.Next(firstNum);
        var operationIndex = _random.Next(_operations.Length);
        var operation = _operations[operationIndex];

        var expressionResult = GetExpressionResult(firstNum, secondNum, operation);
        var question = $"{firstNum} {operation} {secondNum}";

        GameEngine.StartGame(GameName, GameDescription, question, expressionResult.ToString());
    }

    private static int GetExpressionResult(int firstOperand, int secondOperand, string operation)
    {
        switch (operation)
        {
            case "+":
                return firstOperand + secondOperand;
            case "-":
                return firstOperand - secondOperand;
            case "*":
                return firstOperand * secondOperand;
            default:
                throw new ArgumentException($"Unknown operation: {operation}");
        }
    }
}