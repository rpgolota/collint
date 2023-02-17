clear all
global M 

%% Initialization 

M = 2; % Group size 
l = {'d';'o';'n';'a';'l';'g';'e';'r';'b';'t'}; % Letters in puzzle 

%num = {'0';'1';'2';'3';'4';'5';'6';'7';'8';'9'};
%num = [0:9];

% Assign nonrepeating random numbers from 0-9 to the 10 letters in puzzle 
j = randperm(10, 10); 
j = j(1:10)-1; %enforces range of 0-9

for i = 1:10
    l{i,2} = j(i);
end

% Combine letters together from characters in cells in array to numbers in vector 
d = [l{1,2},l{2,2},l{3,2},l{4,2},l{5,2},l{1,2}]; 

r = [l{8,2},l{2,2},l{9,2},l{7,2},l{8,2},l{10,2}];

g = [l{6,2},l{7,2},l{8,2},l{4,2},l{5,2},l{1,2}];

%Stitch numbers together horizontally (side-by-side) and put number in
%vector "names"
a = [d; r; g];

for i = 1:3
    stringX=num2str(a(i,:));
    stringX=stringX(stringX~=' '); % remove the space (this is stupidly complicated)
    names(i,:)=str2num(stringX);
end

donald = names(1);
robert = names(2);
gerald = names(3);

% Define cost function
cost = abs(donald - (robert + gerald));

% Define computational cost
% comp_cost = M*t/factorial(10);



