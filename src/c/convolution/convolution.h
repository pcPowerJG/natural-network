#include "pch.h"
#include <iostream>
#include "Model.h"

using namespace std;

Model *model;
Model *get_model(vector<vector<vector<unsigned>>> conv_param, vector<vector<unsigned>> pool_param, vector<unsigned> nn_param){
	return new Model(conv_param, pool_param, nn_param);
}
int create_model(vector<vector<vector<unsigned>>> conv_param, vector<vector<unsigned>> pool_param, vector<unsigned> nn_param){
	try {
		model = new Model(conv_param, pool_param, nn_param);
		return 0;
	}
	catch (errc e){
		return -1;
	} return 0;
}
double *forward_many_chanell_many_model(vector<vector<vector<double>>> &inputVals, Model *model){
	vector<double> result = model->feedForward(inputVals);
	double *result_ = (double*)malloc(sizeof(double) * result.size());
	for (int i = 0; i < result.size(); i++)
		result_[i] = result.at(i);
	return result_;
}

double *forward_many_chanell(vector<vector<vector<double>>> &inputVals){
	vector<double> result = model->feedForward(inputVals);
	double *result_ = (double*)malloc(sizeof(double) * result.size());
	for (int i = 0; i < result.size(); i++)
		result_[i] = result.at(i);
	delete &result;
	return result_;
}

double *forward_one_channel(vector<vector<double>> &inputVals){
	vector<double> result = model->feedForward(inputVals);
	double *result_ = (double*)malloc(sizeof(double) * result.size());
	for (int i = 0; i < result.size(); i++)
		result_[i] = result.at(i);
	delete &result;
	return result_;
}
